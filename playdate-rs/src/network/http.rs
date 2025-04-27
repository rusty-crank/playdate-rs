use alloc::ffi::CString;

use crate::error::Error;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use no_std_io::io::ErrorKind;
use sys::{accessReply as AccessReply, PDNetErr as NetworkError};

use super::http_handle;

/// Before connecting to a server, permission must be given by the user. Unlike in Lua, we don’t have a way to pause the runtime to present the modal dialog, so this function must be explicitly called before calling http→newConnection(). server can be a parent domain of the connections opened, or NULL to request access to any HTTP server. purpose is an optional string displayed in the permissions dialog to explain why the program is requesting access. After the user responds to the request, requestCallback is called with the given userdata argument
#[allow(static_mut_refs)]
pub fn request_access(
    server: Option<String>,
    port: u16,
    usessl: bool,
    porpose: impl AsRef<str>,
    callback: Box<dyn FnOnce(bool)>,
) -> AccessReply {
    static mut CALLBACK: Option<Box<dyn FnOnce(bool)>> = None;
    unsafe {
        CALLBACK = Some(callback);
    }
    unsafe extern "C" fn callback_impl(allowed: bool, _: *mut core::ffi::c_void) {
        if let Some(cb) = CALLBACK.take() {
            cb(allowed);
        }
    }
    let server_c_string = server.as_ref().map(|s| CString::new(s.as_str()).unwrap());
    let server_ptr = server_c_string
        .as_ref()
        .map(|s| s.as_ptr())
        .unwrap_or(core::ptr::null());
    let porpose_c_string = CString::new(porpose.as_ref()).unwrap();
    let porpose_ptr = porpose_c_string.as_ptr();
    let result = unsafe {
        http_handle().requestAccess.unwrap()(
            server_ptr,
            port as _,
            usessl,
            porpose_ptr,
            Some(callback_impl),
            core::ptr::null_mut(),
        )
    };
    result
}

struct Callbacks {
    header_received: Option<Box<dyn FnOnce(&str, &str)>>,
    headers_read: Option<Box<dyn FnOnce()>>,
    response: Option<Box<dyn FnOnce()>>,
    request_complete: Option<Box<dyn FnOnce()>>,
    connection_closed: Option<Box<dyn FnOnce()>>,
}

pub struct HTTPConnection {
    pub(crate) handle: *mut sys::HTTPConnection,
    closed: bool,
    callbacks: Box<Callbacks>,
}

impl HTTPConnection {
    /// Returns an HTTPConnection object for connecting to the given server, or NULL if permission has been denied or not yet granted. If port is 0, the connection will use port 80 if usessl is false, otherwise 443. No connection is attempted until get() or post() are called.
    pub fn new(server: impl AsRef<str>, port: u16, usessl: bool) -> Result<Self, Error> {
        let server_c_string = CString::new(server.as_ref()).unwrap();
        let server_ptr = server_c_string.as_ptr();
        let handle = unsafe { http_handle().newConnection.unwrap()(server_ptr, port as _, usessl) };
        if handle.is_null() {
            return Err(ErrorKind::PermissionDenied.into());
        }
        let callbacks = Box::new(Callbacks {
            header_received: None,
            headers_read: None,
            response: None,
            request_complete: None,
            connection_closed: None,
        });
        let callbacks_ptr = &*callbacks as *const Callbacks as *mut Callbacks;
        unsafe { http_handle().setUserdata.unwrap()(handle, callbacks_ptr as _) };
        let connection = Self {
            handle,
            closed: true,
            callbacks,
        };
        Ok(connection)
    }

    // Adds 1 to the connection’s retain count, so that it won’t be freed when it scopes out of another context. This is used primarily so we can pass a connection created in Lua into C and not have to worry about the Lua wrapper’s lifespan.
    // pub fn retain(&mut self) {
    //     unsafe { http_handle().retain.unwrap()(self.handle) };
    // }

    /// Sets the length of time (in milliseconds) to wait for the connection to the server to be made.
    pub fn set_timeout(&mut self, ms: usize) {
        unsafe { http_handle().setConnectTimeout.unwrap()(self.handle, ms as _) };
    }

    /// If `keepalive` is true, this causes the HTTP request to include a Connection: keep-alive header.
    pub fn set_keep_alive(&mut self, keep_alive: bool) {
        unsafe { http_handle().setKeepAlive.unwrap()(self.handle, keep_alive) };
    }

    /// Adds a `Range: bytes=<start>-<end>` header to the HTTP request.
    pub fn set_byte_range(&mut self, start: usize, end: usize) {
        unsafe { http_handle().setByteRange.unwrap()(self.handle, start as _, end as _) };
    }

    /// Opens the connection to the server if it’s not already open (e.g. from a previous request with keep-alive enabled) and sends a request with the given method and path, additional headers if specified, and the provided data.
    pub fn query(
        &mut self,
        method: &str,
        path: &str,
        headers: Option<impl AsRef<str>>,
        body: Option<impl AsRef<str>>,
    ) -> Result<(), Error> {
        self.closed = false;
        let method_c_string = CString::new(method).unwrap();
        let method_ptr = method_c_string.as_ptr();
        let path_c_string = CString::new(path).unwrap();
        let path_ptr = path_c_string.as_ptr();
        let headers_c_string = headers.as_ref().map(|s| CString::new(s.as_ref()).unwrap());
        let headers_len = headers_c_string
            .as_ref()
            .map(|s| s.as_bytes().len())
            .unwrap_or(0);
        let headers_ptr = headers_c_string
            .as_ref()
            .map(|s| s.as_ptr())
            .unwrap_or(core::ptr::null());
        let body_c_string = body.as_ref().map(|s| CString::new(s.as_ref()).unwrap());
        let body_len = body_c_string
            .as_ref()
            .map(|s| s.as_bytes().len())
            .unwrap_or(0);
        let body_ptr = body_c_string
            .as_ref()
            .map(|s| s.as_ptr())
            .unwrap_or(core::ptr::null());
        let err = unsafe {
            http_handle().query.unwrap()(
                self.handle,
                method_ptr,
                path_ptr,
                headers_ptr,
                headers_len,
                body_ptr,
                body_len,
            )
        };
        if err == NetworkError::OK {
            Ok(())
        } else {
            Err(Error::NetworkError(err))
        }
    }

    /// Opens the connection to the server if it’s not already open (e.g. from a previous request with keep-alive enabled) and sends a GET request with the given path and additional headers if specified.
    pub fn get(&mut self, path: &str, headers: Option<&str>) -> Result<(), Error> {
        self.closed = false;
        let path_c_string = CString::new(path).unwrap();
        let path_ptr = path_c_string.as_ptr();
        let headers_c_string = headers.as_ref().map(|s| CString::new(*s).unwrap());
        let headers_len = headers_c_string
            .as_ref()
            .map(|s| s.as_bytes().len())
            .unwrap_or(0);
        let headers_ptr = headers_c_string
            .as_ref()
            .map(|s| s.as_ptr())
            .unwrap_or(core::ptr::null());
        let err =
            unsafe { http_handle().get.unwrap()(self.handle, path_ptr, headers_ptr, headers_len) };
        if err == NetworkError::OK {
            Ok(())
        } else {
            Err(Error::NetworkError(err))
        }
    }

    /// Equivalent to calling playdate→network→http→query() with method equal to POST.
    pub fn post(
        &mut self,
        path: &str,
        headers: Option<&str>,
        body: Option<&str>,
    ) -> Result<(), Error> {
        self.closed = false;
        let path_c_string = CString::new(path).unwrap();
        let path_ptr = path_c_string.as_ptr();
        let headers_c_string = headers.as_ref().map(|s| CString::new(*s).unwrap());
        let headers_len = headers_c_string
            .as_ref()
            .map(|s| s.as_bytes().len())
            .unwrap_or(0);
        let headers_ptr = headers_c_string
            .as_ref()
            .map(|s| s.as_ptr())
            .unwrap_or(core::ptr::null());
        let body_c_string = body.as_ref().map(|s| CString::new(*s).unwrap());
        let body_len = body_c_string
            .as_ref()
            .map(|s| s.as_bytes().len())
            .unwrap_or(0);
        let body_ptr = body_c_string
            .as_ref()
            .map(|s| s.as_ptr())
            .unwrap_or(core::ptr::null());
        let err = unsafe {
            http_handle().post.unwrap()(
                self.handle,
                path_ptr,
                headers_ptr,
                headers_len,
                body_ptr,
                body_len,
            )
        };
        if err == NetworkError::OK {
            Ok(())
        } else {
            Err(Error::NetworkError(err))
        }
    }

    /// Returns the last error on the connection
    pub fn get_error(&self) -> Option<NetworkError> {
        let e = unsafe { http_handle().getError.unwrap()(self.handle) };
        if e == NetworkError::OK {
            None
        } else {
            Some(e)
        }
    }

    /// Returns the number of bytes already read from the connection and the total bytes the server plans to send, if known.
    pub fn get_progress(&self) -> ReadProgress {
        let mut received = 0;
        let mut total = 0;
        unsafe {
            http_handle().getProgress.unwrap()(self.handle, &mut received, &mut total);
        }
        ReadProgress {
            received: received as _,
            total: total as _,
        }
    }

    /// Returns the HTTP status response code, if the request response headers have been received and parsed.
    pub fn get_response_code(&self) -> usize {
        let code = unsafe { http_handle().getResponseStatus.unwrap()(self.handle) };
        code as _
    }

    /// Returns the number of bytes currently available for reading from the connection.
    pub fn get_bytes_available(&self) -> usize {
        let bytes = unsafe { http_handle().getBytesAvailable.unwrap()(self.handle) };
        bytes as _
    }

    /// Sets the length of time, in milliseconds, the read() function will wait for incoming data before returning. The default value is 1000, or one second.
    pub fn set_read_timeout(&mut self, ms: usize) {
        unsafe { http_handle().setReadTimeout.unwrap()(self.handle, ms as _) };
    }

    /// Sets the size of the connection’s read buffer. The default buffer size is 64 KB.
    pub fn set_read_buffer_size(&mut self, size: usize) {
        unsafe { http_handle().setReadBufferSize.unwrap()(self.handle, size as _) };
    }

    /// On success, returns up to length bytes (limited by the size of the read buffer) from the connection. If length is more than the number of bytes available the function will wait for more data up to the length of time set by setReadTimeout() (default one second).
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let result = unsafe {
            http_handle().read.unwrap()(self.handle, buf.as_mut_ptr() as *mut _, buf.len() as u32)
        };
        if result >= 0 {
            Ok(result as usize)
        } else {
            Err(ErrorKind::Other.into())
        }
    }

    pub fn read_all(&mut self) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0; self.get_bytes_available()];
        let read = self.read(&mut buf)?;
        assert!(read == buf.len());
        Ok(buf)
    }

    pub fn read_to_string(&mut self) -> Result<String, Error> {
        let buf = self.read_all()?;
        String::from_utf8(buf).map_err(|_| ErrorKind::InvalidData.into())
    }

    /// Sets a callback to be called when the HTTP parser reads a header line from the connection
    pub fn set_header_received_callback(&mut self, callback: Box<dyn FnOnce(&str, &str)>) {
        self.callbacks.header_received = Some(callback);
        unsafe extern "C" fn callback_impl(
            conn: *mut sys::HTTPConnection,
            header: *const core::ffi::c_char,
            value: *const core::ffi::c_char,
        ) {
            let callbacks_ptr =
                unsafe { http_handle().getUserdata.unwrap()(conn) } as *mut Callbacks;
            let callbacks = unsafe { &mut *callbacks_ptr };
            let header = unsafe { ::core::ffi::CStr::from_ptr(header) };
            let value = unsafe { ::core::ffi::CStr::from_ptr(value) };
            let header = header.to_str().unwrap();
            let value = value.to_str().unwrap();
            if let Some(cb) = callbacks.header_received.take() {
                cb(header, value);
            }
        }
        unsafe {
            http_handle().setHeaderReceivedCallback.unwrap()(self.handle, Some(callback_impl))
        };
    }

    /// Sets a function to be called after the connection has parsed the headers from the server response. At this point, getResponseStatus() and getProgress() can be used to query the status and size of the response, and get()/post() can queue another request if connection:setKeepAlive(true) was set and the connection is still open.
    pub fn set_headers_read_callback(&mut self, callback: Box<dyn FnOnce()>) {
        self.callbacks.headers_read = Some(callback);
        unsafe extern "C" fn callback_impl(conn: *mut sys::HTTPConnection) {
            let callbacks_ptr =
                unsafe { http_handle().getUserdata.unwrap()(conn) } as *mut Callbacks;
            let callbacks = unsafe { &mut *callbacks_ptr };
            if let Some(cb) = callbacks.headers_read.take() {
                cb();
            }
        }
        unsafe { http_handle().setHeadersReadCallback.unwrap()(self.handle, Some(callback_impl)) };
    }

    /// Sets a function to be called when data is available for reading.
    #[allow(static_mut_refs)]
    pub fn set_response_callback(&mut self, callback: Box<dyn FnOnce()>) {
        self.callbacks.response = Some(callback);
        unsafe extern "C" fn callback_impl(conn: *mut sys::HTTPConnection) {
            let callbacks_ptr =
                unsafe { http_handle().getUserdata.unwrap()(conn) } as *mut Callbacks;
            let callbacks = unsafe { &mut *callbacks_ptr };
            if let Some(cb) = callbacks.response.take() {
                cb();
            }
        }
        unsafe { http_handle().setResponseCallback.unwrap()(self.handle, Some(callback_impl)) };
    }

    /// Sets a function to be called when all data for the request has been received (if the response contained a Content-Length header and the size is known) or the request times out.
    pub fn set_request_complete_callback(&mut self, callback: Box<dyn FnOnce()>) {
        self.callbacks.request_complete = Some(callback);
        unsafe extern "C" fn callback_impl(conn: *mut sys::HTTPConnection) {
            let callbacks_ptr =
                unsafe { http_handle().getUserdata.unwrap()(conn) } as *mut Callbacks;
            let callbacks = unsafe { &mut *callbacks_ptr };
            if let Some(cb) = callbacks.request_complete.take() {
                cb();
            }
        }
        unsafe {
            http_handle().setRequestCompleteCallback.unwrap()(self.handle, Some(callback_impl))
        };
    }

    /// Sets a function to be called when the server has closed the connection.
    pub fn set_connection_closed_callback(&mut self, callback: Box<dyn FnOnce()>) {
        self.callbacks.connection_closed = Some(callback);
        unsafe extern "C" fn callback_impl(conn: *mut sys::HTTPConnection) {
            let callbacks_ptr =
                unsafe { http_handle().getUserdata.unwrap()(conn) } as *mut Callbacks;
            let callbacks = unsafe { &mut *callbacks_ptr };
            if let Some(cb) = callbacks.connection_closed.take() {
                cb();
            }
        }
        unsafe {
            http_handle().setConnectionClosedCallback.unwrap()(self.handle, Some(callback_impl))
        };
    }
}

pub struct ReadProgress {
    pub received: usize,
    pub total: usize,
}

impl Drop for HTTPConnection {
    fn drop(&mut self) {
        if !self.closed {
            unsafe { http_handle().close.unwrap()(self.handle) };
        }
        unsafe { http_handle().release.unwrap()(self.handle) };
    }
}
