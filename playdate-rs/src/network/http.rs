use alloc::collections::VecDeque;
use alloc::sync::Arc;
use core::{cell::RefCell, future::Future, marker::PhantomData};

use alloc::ffi::CString;
use url::Url;

use crate::alloc::string::ToString;
use crate::error::Error;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};
use no_std_io::io::ErrorKind;
use sys::{accessReply as AccessReply, PDNetErr as NetworkError};

use crate::util::callback_to_async::CallbackFuture;

use super::http_handle;

/// Before connecting to a server, permission must be given by the user. Unlike in Lua, we don’t have a way to pause the runtime to present the modal dialog, so this function must be explicitly called before calling http→newConnection(). server can be a parent domain of the connections opened, or NULL to request access to any HTTP server. purpose is an optional string displayed in the permissions dialog to explain why the program is requesting access. After the user responds to the request, requestCallback is called with the given userdata argument
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
        http_handle().requestAccess.unwrap()(
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
    header_received: Option<Box<dyn FnMut(&str, &str)>>,
    headers_read: Option<Box<dyn FnMut()>>,
    response: Option<Box<dyn FnMut()>>,
    request_complete: Option<Box<dyn FnMut()>>,
    connection_closed: Option<Box<dyn FnMut()>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl HTTPMethod {
    fn as_str(&self) -> &'static str {
        match self {
            HTTPMethod::GET => "GET",
            HTTPMethod::POST => "POST",
            HTTPMethod::PUT => "PUT",
            HTTPMethod::DELETE => "DELETE",
            HTTPMethod::PATCH => "PATCH",
        }
    }
}

pub struct HTTPConnection {
    pub(crate) handle: *mut sys::HTTPConnection,
    closed: bool,
    state: Arc<RefCell<SharedState>>,
    host: Url,
}

macro_rules! impl_http_method {
    ($name:ident, $method:ident) => {
        pub async fn $name<B: serde::Serialize>(
            &mut self,
            path: impl AsRef<str>,
            options: &HTTPOptions<B>,
        ) -> Result<HTTPResponse, Error> {
            let url = self
                .host
                .join(path.as_ref())
                .map_err(|_| ErrorKind::InvalidInput)?;
            let headers = self
                .query(
                    HTTPMethod::$method,
                    path.as_ref(),
                    &options.headers,
                    options.body.as_ref(),
                )
                .await?;
            Ok(HTTPResponse::from_ref(self, &url, headers))
        }
    };
}

impl HTTPConnection {
    /// Returns an HTTPConnection object for connecting to the given server, or NULL if permission has been denied or not yet granted. If port is 0, the connection will use port 80 if usessl is false, otherwise 443. No connection is attempted until get() or post() are called.
    pub fn new(server: impl TryInto<Url>) -> Result<Self, Error> {
        let url: Url = server.try_into().map_err(|_| ErrorKind::InvalidInput)?;
        if url.path() != "/"
            || url.host().is_none()
            || url.scheme() == ""
            || url.query().is_some()
            || url.fragment().is_some()
        {
            println!("Invalid URL: {:?}", url);
            return Err(ErrorKind::InvalidInput.into());
        }
        let host = url.host_str().unwrap_or("");
        let usessl = url.scheme() == "https";
        let port = url.port().unwrap_or(if usessl { 443 } else { 80 });

        let server_c_string = CString::new(host).unwrap();
        let server_ptr = server_c_string.as_ptr();
        let handle = unsafe { http_handle().newConnection.unwrap()(server_ptr, port as _, usessl) };
        if handle.is_null() {
            return Err(ErrorKind::PermissionDenied.into());
        }
        let state = Arc::new(RefCell::new(SharedState {
            closed: true,
            should_release: false,
            header_received: None,
            headers_read: None,
            response: None,
            request_complete: None,
            connection_closed: None,
        }));
        let state_ptr = Box::leak(Box::new(state.clone())) as *mut Arc<RefCell<SharedState>>;
        unsafe { http_handle().setUserdata.unwrap()(handle, state_ptr as _) };
        unsafe extern "C" fn callback_impl(conn: *mut sys::HTTPConnection) {
            let state_ptr = unsafe { http_handle().getUserdata.unwrap()(conn) }
                as *mut Arc<RefCell<SharedState>>;
            let state = unsafe { &*state_ptr };
            let mut state = state.borrow_mut();
            state.closed = true;
            if state.should_release {
                HTTPConnection::release(conn);
                return;
            }
            if let Some(cb) = state.connection_closed.as_mut() {
                cb();
            }
        }
        unsafe { http_handle().setConnectionClosedCallback.unwrap()(handle, Some(callback_impl)) }
        let connection = Self {
            handle,
            closed: true,
            state,
            host: url,
        };
        Ok(connection)
    }

    fn release(conn: *mut sys::HTTPConnection) {
        unsafe {
            let state_ptr =
                http_handle().getUserdata.unwrap()(conn) as *mut Arc<RefCell<SharedState>>;
            let _boxed = Box::from_raw(state_ptr);
            http_handle().setUserdata.unwrap()(conn, core::ptr::null_mut());
            http_handle().release.unwrap()(conn);
        }
    }

    pub fn is_closed(&self) -> bool {
        self.state.borrow().closed
    }

    pub fn close(&mut self) {
        if self.is_closed() {
            return;
        }
        unsafe { http_handle().close.unwrap()(self.handle) };
    }

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

    fn build_headers(headers: &Headers) -> (CString, usize, *const core::ffi::c_char) {
        let mut headers = headers
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\r\n");
        if !headers.is_empty() {
            headers = format!("{}\r\n", headers);
        }
        let c_string = CString::new(headers.as_str()).unwrap();
        let len = c_string.as_bytes().len();
        let ptr = c_string.as_ptr();
        (c_string, len, ptr)
    }

    async fn query<B: serde::Serialize>(
        &mut self,
        method: HTTPMethod,
        path: &str,
        headers: &Headers,
        body: Option<&B>,
    ) -> Result<Headers, Error> {
        // Prepare callbacks
        let future = CallbackFuture::<()>::new();
        let handle = future.get_handle();
        self.set_headers_read_callback(Box::new(move || {
            CallbackFuture::<()>::resolve(handle, ());
        }));
        let res_headers: Arc<RefCell<Headers>> = Default::default();
        let res_headers2 = res_headers.clone();
        self.set_header_received_callback(Box::new(move |k, v| {
            res_headers2.borrow_mut().insert(k, v);
        }));
        // Prepare request arguments
        self.closed = false;
        let path_c_string = CString::new(path).unwrap();
        let path_ptr = path_c_string.as_ptr();
        let (_headers, headers_len, headers_ptr) = Self::build_headers(headers);
        let body = body.as_ref().map(|b| serde_json::to_string(b).unwrap());
        let body_c_string = body.as_ref().map(|s| CString::new(s.as_str()).unwrap());
        let body_len = body_c_string
            .as_ref()
            .map(|s| s.as_bytes().len())
            .unwrap_or(0);
        let body_ptr = body_c_string
            .as_ref()
            .map(|s| s.as_ptr())
            .unwrap_or(core::ptr::null());
        // Send request
        let err = unsafe {
            match method {
                HTTPMethod::GET => {
                    http_handle().get.unwrap()(self.handle, path_ptr, headers_ptr, headers_len)
                }
                HTTPMethod::POST => http_handle().post.unwrap()(
                    self.handle,
                    path_ptr,
                    headers_ptr,
                    headers_len,
                    body_ptr,
                    body_len,
                ),
                m => {
                    let method_c_string = CString::new(m.as_str()).unwrap();
                    let method_ptr = method_c_string.as_ptr();
                    http_handle().query.unwrap()(
                        self.handle,
                        method_ptr,
                        path_ptr,
                        headers_ptr,
                        headers_len,
                        body_ptr,
                        body_len,
                    )
                }
            }
        };
        if err != NetworkError::OK {
            return Err(Error::NetworkError(err));
        }
        self.state.borrow_mut().closed = false;
        // Wait for headers to be read
        future.await;
        // Return headers
        let mut headers = res_headers.borrow_mut();
        Ok(core::mem::take(&mut *headers))
    }

    /// Opens the connection to the server if it’s not already open (e.g. from a previous request with keep-alive enabled) and sends a GET request with the given path and additional headers if specified.
    pub async fn get(
        &mut self,
        path: impl AsRef<str>,
        headers: &Headers,
    ) -> Result<HTTPResponse, Error> {
        let url = self
            .host
            .join(path.as_ref())
            .map_err(|_| ErrorKind::InvalidInput)?;
        let headers = self
            .query::<()>(HTTPMethod::GET, path.as_ref(), headers, None)
            .await?;
        Ok(HTTPResponse::from_ref(self, &url, headers))
    }

    impl_http_method!(post, POST);
    impl_http_method!(put, PUT);
    impl_http_method!(patch, PATCH);
    impl_http_method!(delete, DELETE);

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
    fn get_progress(&self) -> ReadProgress {
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
    fn get_response_code(&self) -> usize {
        let code = unsafe { http_handle().getResponseStatus.unwrap()(self.handle) };
        code as _
    }

    /// Returns the number of bytes currently available for reading from the connection.
    fn get_bytes_available(&self) -> usize {
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
    pub fn read<'a, 'b: 'a>(
        &'a mut self,
        buf: &'b mut [u8],
    ) -> impl 'a + Future<Output = Result<usize, Error>> {
        async move {
            let result = unsafe {
                http_handle().read.unwrap()(
                    self.handle,
                    buf.as_mut_ptr() as *mut _,
                    buf.len() as u32,
                )
            };
            if result >= 0 {
                Ok(result as usize)
            } else {
                Err(ErrorKind::Other.into())
            }
        }
    }

    async fn read_all(&mut self) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0; self.get_bytes_available()];
        let mut cursor = 0;
        while cursor < buf.len() {
            let result = self.read(&mut buf[cursor..]).await?;
            if result == 0 {
                break;
            }
            cursor += result;
        }
        // trim
        buf.truncate(cursor);
        Ok(buf)
    }

    /// Sets a callback to be called when the HTTP parser reads a header line from the connection
    fn set_header_received_callback(&mut self, callback: Box<dyn FnMut(&str, &str)>) {
        self.state.borrow_mut().header_received = Some(callback);
        unsafe extern "C" fn callback_impl(
            conn: *mut sys::HTTPConnection,
            header: *const core::ffi::c_char,
            value: *const core::ffi::c_char,
        ) {
            let state_ptr = unsafe { http_handle().getUserdata.unwrap()(conn) }
                as *mut Arc<RefCell<SharedState>>;
            let state = unsafe { &*state_ptr };
            let mut state = state.borrow_mut();
            let header = unsafe { ::core::ffi::CStr::from_ptr(header) };
            let value = unsafe { ::core::ffi::CStr::from_ptr(value) };
            let header = header.to_str().unwrap();
            let value = value.to_str().unwrap();
            if let Some(cb) = state.header_received.as_mut() {
                cb(header, value);
            }
        }
        unsafe {
            http_handle().setHeaderReceivedCallback.unwrap()(self.handle, Some(callback_impl))
        };
    }

    /// Sets a function to be called after the connection has parsed the headers from the server response. At this point, getResponseStatus() and getProgress() can be used to query the status and size of the response, and get()/post() can queue another request if connection:setKeepAlive(true) was set and the connection is still open.
    fn set_headers_read_callback(&mut self, callback: Box<dyn FnMut()>) {
        self.state.borrow_mut().headers_read = Some(callback);
        unsafe extern "C" fn callback_impl(conn: *mut sys::HTTPConnection) {
            let state_ptr = unsafe { http_handle().getUserdata.unwrap()(conn) }
                as *mut Arc<RefCell<SharedState>>;
            let state = unsafe { &*state_ptr };
            let mut state = state.borrow_mut();
            if let Some(cb) = state.headers_read.as_mut() {
                cb();
            }
        }
        unsafe { http_handle().setHeadersReadCallback.unwrap()(self.handle, Some(callback_impl)) };
    }

    /// Sets a function to be called when data is available for reading.
    #[allow(unused)]
    fn set_response_callback(&mut self, callback: Box<dyn FnMut()>) {
        self.state.borrow_mut().response = Some(callback);
        unsafe extern "C" fn callback_impl(conn: *mut sys::HTTPConnection) {
            let state_ptr = unsafe { http_handle().getUserdata.unwrap()(conn) }
                as *mut Arc<RefCell<SharedState>>;
            let state = unsafe { &*state_ptr };
            let mut state = state.borrow_mut();
            if let Some(cb) = state.response.as_mut() {
                cb();
            }
        }
        unsafe { http_handle().setResponseCallback.unwrap()(self.handle, Some(callback_impl)) };
    }

    /// Sets a function to be called when all data for the request has been received (if the response contained a Content-Length header and the size is known) or the request times out.
    #[allow(unused)]
    fn set_request_complete_callback(&mut self, callback: Box<dyn FnMut()>) {
        self.state.borrow_mut().request_complete = Some(callback);
        unsafe extern "C" fn callback_impl(conn: *mut sys::HTTPConnection) {
            let state_ptr = unsafe { http_handle().getUserdata.unwrap()(conn) }
                as *mut Arc<RefCell<SharedState>>;
            let state = unsafe { &*state_ptr };
            let mut state = state.borrow_mut();
            if let Some(cb) = state.request_complete.as_mut() {
                cb();
            }
        }
        unsafe {
            http_handle().setRequestCompleteCallback.unwrap()(self.handle, Some(callback_impl))
        };
    }

    /// Sets a function to be called when the server has closed the connection.
    pub fn set_connection_closed_callback(&mut self, callback: impl 'static + FnMut()) {
        self.state.borrow_mut().connection_closed = Some(Box::new(callback));
    }
}

pub struct ReadProgress {
    pub received: usize,
    pub total: usize,
}

impl Drop for HTTPConnection {
    fn drop(&mut self) {
        if self.is_closed() {
            HTTPConnection::release(self.handle);
        } else {
            self.state.borrow_mut().should_release = true;
            self.close();
        }
    }
}

enum RefMutOrOwned<'a, T> {
    Ref(&'a mut T),
    Owned(T),
}

impl Deref for RefMutOrOwned<'_, HTTPConnection> {
    type Target = HTTPConnection;

    fn deref(&self) -> &Self::Target {
        match self {
            RefMutOrOwned::Ref(conn) => conn,
            RefMutOrOwned::Owned(conn) => conn,
        }
    }
}

impl DerefMut for RefMutOrOwned<'_, HTTPConnection> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            RefMutOrOwned::Ref(conn) => conn,
            RefMutOrOwned::Owned(conn) => conn,
        }
    }
}

pub struct HTTPResponse<'a> {
    url: Url,
    conn: RefMutOrOwned<'a, HTTPConnection>,
    status_code: usize,
    headers: Headers,
}

impl<'a> HTTPResponse<'a> {
    fn from_ref(conn: &'a mut HTTPConnection, path: &Url, res_headers: Headers) -> Self {
        let status_code = conn.get_response_code();
        let mut url = conn.host.clone();
        url.set_path(path.path());
        url.set_query(path.query());
        Self {
            conn: RefMutOrOwned::Ref(conn),
            headers: res_headers,
            status_code,
            url,
        }
    }
    fn from_owned(conn: HTTPConnection, path: &Url, res_headers: Headers) -> Self {
        let status_code = conn.get_response_code();
        let mut url = conn.host.clone();
        url.set_path(path.path());
        url.set_query(path.query());
        Self {
            conn: RefMutOrOwned::Owned(conn),
            headers: res_headers,
            status_code,
            url,
        }
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn status_code(&self) -> usize {
        self.status_code
    }

    pub fn is_ok(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }

    async fn read_all(&mut self) -> Result<Vec<u8>, Error> {
        self.conn.read_all().await
    }

    pub async fn data(mut self) -> Result<Vec<u8>, Error> {
        self.read_all().await
    }

    pub async fn string(self) -> Result<String, Error> {
        let data = self.data().await?;
        let result = core::str::from_utf8(&data);
        match result {
            Ok(value) => Ok(value.to_owned()),
            Err(_) => Err(ErrorKind::InvalidData.into()),
        }
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn get_header(&self, key: &str) -> Option<&str> {
        self.headers.get(key)
    }

    pub async fn json<T: serde::de::DeserializeOwned>(self) -> Result<T, Error> {
        let data = self.string().await?;
        let result = serde_json::from_str(&data);
        match result {
            Ok(value) => Ok(value),
            Err(_e) => Err(ErrorKind::InvalidData.into()),
        }
    }

    pub fn stream(self) -> HTTPEventStream<'a, Vec<u8>, HTTPRawEventStream<'a>> {
        HTTPEventStream {
            stream: HTTPRawEventStream { res: self },
            map: MapFunc::Ref(&|s| Ok(s)),
            p: PhantomData,
            filter: None,
        }
    }

    pub fn read_progress(&self) -> ReadProgress {
        self.conn.get_progress()
    }

    pub fn bytes_available(&self) -> usize {
        self.conn.get_bytes_available()
    }
}

pub trait AsyncStream<'a> {
    type Item;

    #[allow(async_fn_in_trait)]
    async fn next(&mut self) -> Result<Self::Item, Error>;
}

pub struct HTTPRawEventStream<'a> {
    res: HTTPResponse<'a>,
}

impl<'a> AsyncStream<'a> for HTTPRawEventStream<'a> {
    type Item = Vec<u8>;

    async fn next(&mut self) -> Result<Vec<u8>, Error> {
        loop {
            if self.res.conn.get_bytes_available() > 0 {
                break;
            }
            crate::PLAYDATE.yield_now().await;
        }
        let data = self.res.conn.read_all().await?;
        Ok(data)
    }
}

enum MapFunc<'a, T, U> {
    Owned(Box<dyn Fn(T) -> Result<U, Error>>),
    Ref(&'a dyn Fn(T) -> Result<U, Error>),
}

impl<'a, T, U> MapFunc<'a, T, U> {
    fn call(&self, value: T) -> Result<U, Error> {
        match self {
            MapFunc::Owned(f) => f(value),
            MapFunc::Ref(f) => f(value),
        }
    }
}

pub struct HTTPEventStream<'a, T: 'static, S: AsyncStream<'a>> {
    stream: S,
    map: MapFunc<'a, S::Item, T>,
    filter: Option<Box<dyn Fn(&T) -> bool>>,
    p: PhantomData<&'a T>,
}

impl<'a, T, S: AsyncStream<'a>> AsyncStream<'a> for HTTPEventStream<'a, T, S> {
    type Item = T;
    async fn next(&mut self) -> Result<T, Error> {
        loop {
            let value = self.stream.next().await?;
            let mapped_value = self.map.call(value)?;
            if let Some(ref filter) = self.filter {
                if !filter(&mapped_value) {
                    continue;
                }
            }
            return Ok(mapped_value);
        }
    }
}

impl<'a, T: 'static, S: AsyncStream<'a>> HTTPEventStream<'a, T, S> {
    pub fn map<U: 'a>(
        self,
        f: impl 'static + Fn(T) -> Result<U, Error>,
    ) -> HTTPEventStream<'a, U, Self> {
        HTTPEventStream {
            stream: self,
            map: MapFunc::Owned(Box::new(f)),
            filter: None,
            p: PhantomData,
        }
    }

    pub fn filter(mut self, f: impl 'static + Fn(&T) -> bool) -> Self
    where
        T: 'static,
    {
        if self.filter.is_none() {
            self.filter = Some(Box::new(f));
        } else {
            let filter = self.filter.take().unwrap();
            self.filter = Some(Box::new(move |item| filter(&item) && f(item)));
        }
        self
    }
}

impl<'a, S: AsyncStream<'a>> HTTPEventStream<'a, Vec<u8>, S> {
    pub fn string(self) -> HTTPEventStream<'a, String, Self> {
        self.map(|s| {
            let result = core::str::from_utf8(&s);
            match result {
                Ok(value) => Ok(value.to_owned()),
                Err(_) => Err(ErrorKind::InvalidData.into()),
            }
        })
    }
}

impl<'a, S: AsyncStream<'a>> HTTPEventStream<'a, Vec<u8>, S> {
    pub fn map_json<U: serde::de::DeserializeOwned>(
        self,
    ) -> HTTPEventStream<'a, U, HTTPEventStream<'a, String, HTTPEventStream<'a, Vec<u8>, S>>> {
        self.string().map_json()
    }
}

impl<'a, S: AsyncStream<'a>> HTTPEventStream<'a, String, S> {
    pub fn map_json<U: serde::de::DeserializeOwned>(self) -> HTTPEventStream<'a, U, Self> {
        HTTPEventStream {
            stream: self,
            map: MapFunc::Owned(Box::new(|s| {
                let result = serde_json::from_str(&s);
                match result {
                    Ok(value) => Ok(value),
                    Err(_e) => Err(ErrorKind::InvalidData.into()),
                }
            })),
            filter: None,
            p: PhantomData,
        }
    }
}

impl<'a, T, S: AsyncStream<'a>> HTTPEventStream<'a, Vec<T>, S> {
    pub fn flatten(self) -> HTTPEventStream<'a, T, HTTPFlattenedEventStream<'a, T, Self>> {
        let f = HTTPFlattenedEventStream {
            stream: self,
            buf: VecDeque::new(),
            p: PhantomData,
        };
        HTTPEventStream {
            stream: f,
            map: MapFunc::Ref(&|s| Ok(s)),
            filter: None,
            p: PhantomData,
        }
    }
}

pub struct HTTPFlattenedEventStream<'a, T, S: AsyncStream<'a, Item = Vec<T>>> {
    stream: S,
    buf: VecDeque<T>,
    p: PhantomData<&'a T>,
}

impl<'a, T: 'static, S: AsyncStream<'a, Item = Vec<T>>> AsyncStream<'a>
    for HTTPFlattenedEventStream<'a, T, S>
{
    type Item = T;
    async fn next(&mut self) -> Result<T, Error> {
        if let Some(value) = self.buf.pop_front() {
            return Ok(value);
        }
        loop {
            let buf = self.stream.next().await?;
            if buf.is_empty() {
                continue;
            }
            for value in buf {
                self.buf.push_back(value);
            }
            if let Some(value) = self.buf.pop_front() {
                return Ok(value);
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct Headers {
    headers: BTreeMap<String, String>,
}

impl core::fmt::Debug for Headers {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.headers)?;
        Ok(())
    }
}

impl Headers {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with(mut self, key: &str, value: &str) -> Self {
        self.insert(key, value);
        self
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|s| s.as_str())
    }
}

#[derive(Default, Clone)]
pub struct HTTPOptions<Body: serde::Serialize = ()> {
    pub headers: Headers,
    pub body: Option<Body>,
}

impl<Body: serde::Serialize> HTTPOptions<Body> {
    pub fn new() -> Self {
        Self {
            headers: Headers::new(),
            body: None,
        }
    }
}

pub async fn get<'a>(url: impl TryInto<Url>, headers: &Headers) -> Result<HTTPResponse<'a>, Error> {
    let url: Url = url.try_into().map_err(|_| ErrorKind::InvalidInput)?;
    let mut host = url.clone();
    host.set_path("/");
    host.set_query(None);
    let mut conn = HTTPConnection::new(host)?;
    let path = url.path().to_string();
    let headers = conn
        .query::<()>(HTTPMethod::GET, path.as_ref(), headers, None)
        .await?;
    Ok(HTTPResponse::from_owned(conn, &url, headers))
}

macro_rules! impl_http_method2 {
    ($name:ident, $method:ident) => {
        pub async fn $name<'a, Body: serde::Serialize>(
            url: impl TryInto<Url>,
            options: &HTTPOptions<Body>,
        ) -> Result<HTTPResponse<'a>, Error> {
            let url: Url = url.try_into().map_err(|_| ErrorKind::InvalidInput)?;
            let mut host = url.clone();
            host.set_path("/");
            host.set_query(None);
            let mut conn = HTTPConnection::new(host)?;
            let path = url.path().to_string();
            let headers = conn
                .query(
                    HTTPMethod::$method,
                    path.as_ref(),
                    &options.headers,
                    options.body.as_ref(),
                )
                .await?;
            Ok(HTTPResponse::from_owned(conn, &url, headers))
        }
    };
}

impl_http_method2!(post, POST);
impl_http_method2!(put, PUT);
impl_http_method2!(patch, PATCH);
impl_http_method2!(delete, DELETE);
