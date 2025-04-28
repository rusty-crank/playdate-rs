use core::{ffi::c_void, future::Future};

use alloc::{ffi::CString, vec, vec::Vec};
use url::Url;

use crate::{error::Error, util::callback_to_async::CallbackFuture};
use alloc::boxed::Box;
use alloc::string::String;
use no_std_io::io::ErrorKind;
use sys::{accessReply as AccessReply, PDNetErr as NetworkError};

use super::tcp_handle;

/// Before connecting to a server, permission must be given by the user. Unlike in Lua, we don’t have a way to pause the runtime to present the modal dialog, so this function must be explicitly called before calling newConnection(). server can be a parent domain of the connections opened, or NULL to request access to any HTTP server. Similarly, if port is zero, this requests access to all ports on the target server(s). purpose is an optional string displayed in the permissions dialog to explain why the program is requesting access. After the user responds to the request, requestCallback is called with the given userdata argument.
pub fn request_access(
    server: Option<String>,
    port: u16,
    usessl: bool,
    porpose: impl AsRef<str>,
) -> impl Future<Output = AccessReply> {
    let future = CallbackFuture::<AccessReply>::new();
    unsafe extern "C" fn callback_impl(allowed: bool, data: *mut core::ffi::c_void) {
        CallbackFuture::<AccessReply>::resolve(
            data,
            if allowed {
                AccessReply::Allow
            } else {
                AccessReply::Deny
            },
        );
    }
    let server_c_string = server.as_ref().map(|s| CString::new(s.as_str()).unwrap());
    let server_ptr = server_c_string
        .as_ref()
        .map(|s| s.as_ptr())
        .unwrap_or(core::ptr::null());
    let porpose_c_string = CString::new(porpose.as_ref()).unwrap();
    let porpose_ptr = porpose_c_string.as_ptr();
    let result = unsafe {
        tcp_handle().requestAccess.unwrap()(
            server_ptr,
            port as _,
            usessl,
            porpose_ptr,
            Some(callback_impl),
            future.get_handle(),
        )
    };
    if result == AccessReply::Ask {
        future
    } else {
        future.set_result(result);
        future
    }
}

struct Callbacks {
    connection_closed: Option<Box<Box<dyn FnMut()>>>,
}

pub struct TCPConnection {
    handle: *mut sys::TCPConnection,
    callbacks: Box<Callbacks>,
}

impl TCPConnection {
    /// Returns a playdate.network.tcp object for connecting to the given server, or NULL if permission has been denied or not yet granted. No connection is attempted until open() is called.
    pub fn new(server: impl TryInto<Url>, ssl: bool) -> Result<Self, Error> {
        let url: Url = server.try_into().map_err(|_| ErrorKind::InvalidInput)?;
        if url.path() != "/"
            || url.host().is_none()
            || url.scheme() == ""
            || url.query().is_some()
            || url.fragment().is_some()
        {
            return Err(ErrorKind::InvalidInput.into());
        }
        let host = url.host_str().unwrap_or("");
        let port = url.port().unwrap_or(if ssl { 443 } else { 80 });

        let server_c_string = CString::new(host).unwrap();
        let server_ptr = server_c_string.as_ptr();
        let handle = unsafe { tcp_handle().newConnection.unwrap()(server_ptr, port as _, ssl) };
        if handle.is_null() {
            return Err(ErrorKind::PermissionDenied.into());
        }
        let callbacks = Box::new(Callbacks {
            connection_closed: None,
        });
        let callbacks_ptr = &*callbacks as *const Callbacks as *mut Callbacks;
        unsafe { tcp_handle().setUserdata.unwrap()(handle, callbacks_ptr as _) };
        let connection = Self { handle, callbacks };
        Ok(connection)
    }

    /// Returns the last error on the connection
    pub fn get_error(&self) -> Option<NetworkError> {
        let e = unsafe { tcp_handle().getError.unwrap()(self.handle) };
        if e == NetworkError::OK {
            None
        } else {
            Some(e)
        }
    }

    /// Sets the length of time (in milliseconds) to wait for the connection to the server to be made.
    pub fn set_timeout(&mut self, ms: usize) {
        unsafe { tcp_handle().setConnectTimeout.unwrap()(self.handle, ms as _) };
    }

    /// Attempts to open the connection to the server. Note that an error may be returned immediately, or in the open callback depending on where it occurs.
    pub async fn open(&mut self) -> Result<(), Error> {
        let future = CallbackFuture::<NetworkError>::new();
        let handle = future.get_handle();
        extern "C" fn callback_impl(
            _conn: *mut sys::TCPConnection,
            error: NetworkError,
            handle: *mut c_void,
        ) {
            CallbackFuture::<NetworkError>::resolve(handle, error);
        }
        let result =
            unsafe { tcp_handle().open.unwrap()(self.handle, Some(callback_impl), handle) };

        if result != NetworkError::OK {
            future.set_result(result);
        }
        let result = future.await;
        if result != NetworkError::OK {
            return Err(Error::NetworkError(result));
        }
        Ok(())
    }

    /// Sets a callback to be called when the connection is closed.
    pub fn set_connection_closed_callback(&mut self, callback: Box<dyn FnMut()>) {
        self.callbacks.connection_closed = Some(Box::new(callback));
        unsafe extern "C" fn callback_impl(conn: *mut sys::TCPConnection, _e: NetworkError) {
            let callbacks_ptr =
                unsafe { tcp_handle().getUserdata.unwrap()(conn) } as *mut Callbacks;
            let callbacks = unsafe { &mut *callbacks_ptr };
            if let Some(mut cb) = callbacks.connection_closed.take() {
                cb();
            }
        }
        unsafe {
            tcp_handle().setConnectionClosedCallback.unwrap()(self.handle, Some(callback_impl))
        };
    }

    /// Sets the length of time, in milliseconds, read() will wait for incoming data before returning. The default value is 1000, or one second.
    pub fn set_read_timeout(&mut self, ms: usize) {
        unsafe { tcp_handle().setReadTimeout.unwrap()(self.handle, ms as _) };
    }

    /// Sets the size of the connection’s read buffer. The default buffer size is 64 KB.
    pub fn set_read_buffer_size(&mut self, bytes: usize) {
        unsafe { tcp_handle().setReadBufferSize.unwrap()(self.handle, bytes as _) };
    }

    /// Returns the number of bytes currently available for reading from the connection.
    pub fn get_bytes_available(&self) -> usize {
        unsafe { tcp_handle().getBytesAvailable.unwrap()(self.handle) as _ }
    }

    /// Attempts to read up to length bytes from the connection into buffer. If length is more than the number of bytes available on the connection the function will wait for more data, up to the length of time set by setReadTimeout() (default one second). Returns the number of bytes actually read, or a (negative) PDNetErr value on error.
    pub fn recv<'a, 'b: 'a>(
        &'a mut self,
        buf: &'b mut [u8],
    ) -> impl 'a + Future<Output = Result<usize, Error>> {
        async move {
            let result = unsafe {
                tcp_handle().read.unwrap()(self.handle, buf.as_mut_ptr() as *mut _, buf.len())
            };
            if result >= 0 {
                Ok(result as usize)
            } else {
                Err(ErrorKind::Other.into())
            }
        }
    }

    /// Attempts to write up to length bytes to the connection. Returns the number of bytes actually written, which may be less than length, or a (negative) PDNetErr value on error.
    pub fn send<'a, 'b: 'a>(
        &'a mut self,
        buf: &'b [u8],
    ) -> impl 'a + Future<Output = Result<usize, Error>> {
        async move {
            let result = unsafe {
                tcp_handle().write.unwrap()(self.handle, buf.as_ptr() as *const _, buf.len())
            };
            if result >= 0 {
                Ok(result as usize)
            } else {
                Err(ErrorKind::Other.into())
            }
        }
    }

    pub async fn wait_for_data(&mut self) {
        loop {
            if self.get_bytes_available() > 0 {
                break;
            }
            crate::PLAYDATE.yield_now().await;
        }
    }

    pub async fn recv_all(&mut self) -> Result<Vec<u8>, Error> {
        if self.get_bytes_available() == 0 {
            return Ok(vec![]);
        }
        let mut buf = vec![0; self.get_bytes_available()];
        self.recv(&mut buf).await?;
        Ok(buf)
    }
}

impl Drop for TCPConnection {
    fn drop(&mut self) {
        // Looks like releasing a connection will use-after-free on the handle
        // unsafe { tcp_handle().release.unwrap()(self.handle) }
    }
}
