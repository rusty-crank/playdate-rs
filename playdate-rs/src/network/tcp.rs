use core::{ffi::c_void, future::Future};

use alloc::{ffi::CString, vec, vec::Vec};
use spin::Mutex;
use url::Url;

use crate::{error::Error, util::callback_to_async::CallbackFuture};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
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

struct SharedState {
    closed: bool,
    should_release: bool,
    connection_closed: Option<Box<dyn FnMut()>>,
}

pub struct TCPConnection {
    handle: *mut sys::TCPConnection,
    state: Arc<Mutex<SharedState>>,
}

unsafe impl Send for TCPConnection {}
unsafe impl Sync for TCPConnection {}

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
        let state = Arc::new(Mutex::new(SharedState {
            closed: true,
            should_release: false,
            connection_closed: None,
        }));
        let state_ptr = Box::leak(Box::new(state.clone())) as *mut Arc<Mutex<SharedState>>;
        unsafe { tcp_handle().setUserdata.unwrap()(handle, state_ptr as _) };
        unsafe extern "C" fn callback_impl(conn: *mut sys::TCPConnection, _e: NetworkError) {
            let state_ptr =
                unsafe { tcp_handle().getUserdata.unwrap()(conn) } as *mut Arc<Mutex<SharedState>>;
            let state = unsafe { &*state_ptr };
            let mut state = state.lock();
            state.closed = true;
            if state.should_release {
                TCPConnection::release(conn);
                return;
            }
            if let Some(cb) = state.connection_closed.as_mut() {
                cb();
            }
        }
        unsafe { tcp_handle().setConnectionClosedCallback.unwrap()(handle, Some(callback_impl)) }
        let connection = Self { handle, state };
        Ok(connection)
    }

    fn release(conn: *mut sys::TCPConnection) {
        unsafe {
            let state_ptr = tcp_handle().getUserdata.unwrap()(conn) as *mut Arc<Mutex<SharedState>>;
            let _boxed = Box::from_raw(state_ptr);
            tcp_handle().setUserdata.unwrap()(conn, core::ptr::null_mut());
            tcp_handle().release.unwrap()(conn);
        }
    }

    pub fn is_closed(&self) -> bool {
        self.state.lock().closed
    }

    pub fn close(&self) {
        if self.is_closed() {
            return;
        }
        unsafe { tcp_handle().close.unwrap()(self.handle) };
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
    pub fn set_timeout(&self, ms: usize) {
        unsafe { tcp_handle().setConnectTimeout.unwrap()(self.handle, ms as _) };
    }

    /// Attempts to open the connection to the server. Note that an error may be returned immediately, or in the open callback depending on where it occurs.
    pub async fn open(&self) -> Result<(), Error> {
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
        self.state.lock().closed = false;
        if result != NetworkError::OK {
            return Err(Error::NetworkError(result));
        }
        Ok(())
    }

    /// Sets a callback to be called when the connection is closed.
    pub fn set_connection_closed_callback(&self, callback: impl 'static + FnMut()) {
        self.state.lock().connection_closed = Some(Box::new(callback));
    }

    /// Sets the length of time, in milliseconds, read() will wait for incoming data before returning. The default value is 1000, or one second.
    pub fn set_read_timeout(&self, ms: usize) {
        unsafe { tcp_handle().setReadTimeout.unwrap()(self.handle, ms as _) };
    }

    /// Sets the size of the connection’s read buffer. The default buffer size is 64 KB.
    pub fn set_read_buffer_size(&self, bytes: usize) {
        unsafe { tcp_handle().setReadBufferSize.unwrap()(self.handle, bytes as _) };
    }

    /// Returns the number of bytes currently available for reading from the connection.
    pub fn get_bytes_available(&self) -> usize {
        unsafe { tcp_handle().getBytesAvailable.unwrap()(self.handle) as _ }
    }

    /// Attempts to read up to length bytes from the connection into buffer. If length is more than the number of bytes available on the connection the function will wait for more data, up to the length of time set by setReadTimeout() (default one second). Returns the number of bytes actually read, or a (negative) PDNetErr value on error.
    pub fn recv<'a, 'b: 'a>(
        &'a self,
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
        &'a self,
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

    pub async fn wait_for_data(&self) {
        loop {
            if self.get_bytes_available() > 0 {
                break;
            }
            crate::PLAYDATE.yield_now().await;
        }
    }

    pub async fn recv_all(&self) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0; self.get_bytes_available()];
        let mut cursor = 0;
        while cursor < buf.len() {
            let result = self.recv(&mut buf[cursor..]).await?;
            if result == 0 {
                break;
            }
            cursor += result;
        }
        // trim
        buf.truncate(cursor);
        Ok(buf)
    }
}

impl Drop for TCPConnection {
    fn drop(&mut self) {
        if self.is_closed() {
            TCPConnection::release(self.handle);
        } else {
            self.state.lock().should_release = true;
            self.close();
        }
    }
}
