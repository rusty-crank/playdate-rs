use alloc::ffi::CString;
use alloc::sync::Arc;
use core::ffi::{c_char, c_void, CStr};
use spin::Mutex;

use crate::PLAYDATE;

struct MenuItemPayload {
    handle: Mutex<*mut sys::PDMenuItem>,
    handler: Mutex<Option<Box<dyn FnMut()>>>,
}

pub struct MenuItem {
    handle: *mut sys::PDMenuItem,
    payload: Arc<MenuItemPayload>,
}

unsafe impl Send for MenuItem {}
unsafe impl Sync for MenuItem {}

impl MenuItem {
    pub(crate) fn new() -> Self {
        MenuItem {
            handle: core::ptr::null_mut(),
            payload: Arc::new(MenuItemPayload {
                handle: Mutex::new(core::ptr::null_mut()),
                handler: Mutex::new(None),
            }),
        }
    }

    pub(crate) fn set_handle(&mut self, handle: *mut sys::PDMenuItem) {
        self.handle = handle;
        *self.payload.handle.lock() = handle;
    }

    pub(crate) fn payload_ptr(&self) -> *mut c_void {
        let payload: &MenuItemPayload = self.payload.as_ref();
        let payload_ptr: *mut MenuItemPayload = payload as *const _ as *mut _;
        payload_ptr as *mut c_void
    }

    pub(crate) extern "C" fn callback(payload: *mut c_void) {
        let payload: &MenuItemPayload = unsafe { &*(payload as *const MenuItemPayload) };
        let mut handler = payload.handler.lock();
        if let Some(ref mut f) = *handler {
            f();
        }
    }

    /// Gets the integer value of the menu item.
    ///
    /// For checkmark menu items, 1 means checked, 0 unchecked. For option menu items, the value indicates the array index of the currently selected option.
    pub fn get_value(&self) -> i32 {
        unsafe { (*PLAYDATE.system.handle).getMenuItemValue.unwrap()(self.handle) }
    }

    /// Sets the integer value of the menu item.
    ///
    /// For checkmark menu items, 1 means checked, 0 unchecked. For option menu items, the value indicates the array index of the currently selected option.
    pub fn set_value(&self, value: i32) {
        unsafe { (*PLAYDATE.system.handle).setMenuItemValue.unwrap()(self.handle, value) }
    }

    /// Gets the display title of the menu item.
    pub fn get_title(&self) -> &str {
        let c_buf = unsafe { (*PLAYDATE.system.handle).getMenuItemTitle.unwrap()(self.handle) };
        let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
        let s: &str = c_str.to_str().unwrap();
        s
    }

    /// Sets the display title of the menu item.
    pub fn set_title(&self, title: impl AsRef<str>) {
        let c_string = CString::new(title.as_ref()).unwrap();
        unsafe {
            (*PLAYDATE.system.handle).setMenuItemTitle.unwrap()(
                self.handle,
                c_string.as_ptr() as *mut c_char,
            )
        }
    }

    pub fn set_handler(&mut self, handler: impl 'static + FnMut()) {
        *self.payload.handler.lock() = Some(Box::new(handler));
    }
}

impl Drop for MenuItem {
    fn drop(&mut self) {
        unsafe { (*PLAYDATE.system.handle).removeMenuItem.unwrap()(self.handle) }
    }
}
