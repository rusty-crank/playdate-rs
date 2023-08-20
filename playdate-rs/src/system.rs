use core::ffi::{c_char, c_void, CStr};

use alloc::ffi::CString;
pub use sys::{
    PDButtons as Buttons, PDDateTime as DateTime, PDLanguage as Language,
    PDPeripherals as Peripherals, PDSystemEvent as SystemEvent,
};

use crate::PLAYDATE;

pub struct System {
    handle: *const sys::playdate_sys,
}

impl System {
    pub(crate) fn new(handle: *const sys::playdate_sys) -> Self {
        System { handle }
    }

    /// ptr = NULL -> malloc, size = 0 -> free
    pub(crate) fn realloc(&self, ptr: *mut c_void, size: usize) -> *mut c_void {
        unsafe { (*self.handle).realloc.unwrap()(ptr, size) }
    }

    pub fn log_to_console(&self, msg: impl AsRef<str>) {
        unsafe {
            let c_string = CString::new(msg.as_ref()).unwrap();
            (*self.handle).logToConsole.unwrap()(c_string.as_ptr() as *mut c_char);
        }
    }

    pub fn error(&self, msg: impl AsRef<str>) {
        unsafe {
            let c_string = CString::new(msg.as_ref()).unwrap();
            (*self.handle).error.unwrap()(c_string.as_ptr() as *mut c_char);
        }
    }

    pub fn get_language(&self) -> Language {
        unsafe { (*self.handle).getLanguage.unwrap()() }
    }

    pub fn get_current_time_milliseconds(&self) -> usize {
        unsafe { (*self.handle).getCurrentTimeMilliseconds.unwrap()() as _ }
    }

    pub fn get_seconds_since_epoch(&self) -> (usize, usize) {
        let mut ms = 0;
        unsafe {
            let s = (*self.handle).getSecondsSinceEpoch.unwrap()(&mut ms);
            (s as _, ms as _)
        }
    }

    pub fn draw_fps(&self, x: i32, y: i32) {
        unsafe { (*self.handle).drawFPS.unwrap()(x, y) }
    }

    pub(crate) fn set_update_callback(&self, update: sys::PDCallbackFunction) {
        unsafe {
            (*self.handle).setUpdateCallback.unwrap()(update, core::ptr::null_mut());
        }
    }

    pub fn get_button_state(&self) -> (Buttons, Buttons, Buttons) {
        let mut buttons = (Buttons(0), Buttons(0), Buttons(0));
        unsafe {
            (*self.handle).getButtonState.unwrap()(&mut buttons.0, &mut buttons.1, &mut buttons.2);
            buttons
        }
    }

    pub fn set_peripherals_enabled(&self, mask: Peripherals) {
        unsafe {
            (*self.handle).setPeripheralsEnabled.unwrap()(mask);
        }
    }

    pub fn get_accelerometer(&self) -> (f32, f32, f32) {
        let x = core::ptr::null_mut();
        let y = core::ptr::null_mut();
        let z = core::ptr::null_mut();
        unsafe {
            (*self.handle).getAccelerometer.unwrap()(x, y, z);
            (*x, *y, *z)
        }
    }

    pub fn get_crank_angle(&self) -> f32 {
        unsafe { (*self.handle).getCrankAngle.unwrap()() }
    }

    pub fn get_crank_change(&self) -> f32 {
        unsafe { (*self.handle).getCrankChange.unwrap()() }
    }

    pub fn is_crank_docked(&self) -> bool {
        unsafe {
            let result = (*self.handle).isCrankDocked.unwrap()();
            result == 1
        }
    }

    pub fn set_crank_sounds_disabled(&self, flag: bool) -> bool {
        unsafe {
            let result = (*self.handle).setCrankSoundsDisabled.unwrap()(flag as i32);
            result == 1
        }
    }

    pub fn get_flipped(&self) -> bool {
        unsafe {
            let result = (*self.handle).getFlipped.unwrap()();
            result == 1
        }
    }

    pub fn set_auto_lock_disabled(&self, disable: bool) {
        unsafe { (*self.handle).setAutoLockDisabled.unwrap()(disable as i32) }
    }

    pub fn set_menu_image(&self, bitmap: *mut sys::LCDBitmap, x_offset: i32) {
        unsafe { (*self.handle).setMenuImage.unwrap()(bitmap, x_offset) }
    }

    pub fn add_menu_item(
        &self,
        title: impl AsRef<str>,
        callback: sys::PDMenuItemCallbackFunction,
        userdata: *mut c_void,
    ) -> MenuItem {
        MenuItem::new(unsafe {
            let c_string = CString::new(title.as_ref()).unwrap();
            (*self.handle).addMenuItem.unwrap()(
                c_string.as_ptr() as *mut c_char,
                callback,
                userdata,
            )
        })
    }

    pub fn add_checkmark_menu_item(
        &self,
        title: impl AsRef<str>,
        value: i32,
        callback: sys::PDMenuItemCallbackFunction,
        userdata: *mut c_void,
    ) -> MenuItem {
        MenuItem::new(unsafe {
            let c_string = CString::new(title.as_ref()).unwrap();
            (*self.handle).addCheckmarkMenuItem.unwrap()(
                c_string.as_ptr() as *mut c_char,
                value,
                callback,
                userdata,
            )
        })
    }

    pub fn add_options_menu_item(
        &self,
        title: impl AsRef<str>,
        option_titles: *mut *const c_char,
        options_count: i32,
        callback: sys::PDMenuItemCallbackFunction,
        userdata: *mut c_void,
    ) -> MenuItem {
        MenuItem::new(unsafe {
            let c_string = CString::new(title.as_ref()).unwrap();
            (*self.handle).addOptionsMenuItem.unwrap()(
                c_string.as_ptr() as *mut c_char,
                option_titles,
                options_count,
                callback,
                userdata,
            )
        })
    }

    pub fn remove_all_menu_items(&self) {
        unsafe { (*self.handle).removeAllMenuItems.unwrap()() }
    }

    pub(crate) fn remove_menu_item(&self, menu_item: *mut sys::PDMenuItem) {
        unsafe { (*self.handle).removeMenuItem.unwrap()(menu_item) }
    }

    pub(crate) fn get_menu_item_value(&self, menu_item: *mut sys::PDMenuItem) -> i32 {
        unsafe { (*self.handle).getMenuItemValue.unwrap()(menu_item) }
    }

    pub(crate) fn set_menu_item_value(&self, menu_item: *mut sys::PDMenuItem, value: i32) {
        unsafe { (*self.handle).setMenuItemValue.unwrap()(menu_item, value) }
    }

    pub(crate) fn get_menu_item_title(&self, menu_item: *mut sys::PDMenuItem) -> *const c_char {
        unsafe { (*self.handle).getMenuItemTitle.unwrap()(menu_item) }
    }

    pub(crate) fn set_menu_item_title(
        &self,
        menu_item: *mut sys::PDMenuItem,
        title: impl AsRef<str>,
    ) {
        unsafe {
            let c_string = CString::new(title.as_ref()).unwrap();
            (*self.handle).setMenuItemTitle.unwrap()(menu_item, c_string.as_ptr() as *mut c_char)
        }
    }

    pub(crate) fn get_menu_item_userdata(&self, menu_item: *mut sys::PDMenuItem) -> *mut c_void {
        unsafe { (*self.handle).getMenuItemUserdata.unwrap()(menu_item) }
    }

    pub(crate) fn set_menu_item_userdata(
        &self,
        menu_item: *mut sys::PDMenuItem,
        userdata: *mut c_void,
    ) {
        unsafe { (*self.handle).setMenuItemUserdata.unwrap()(menu_item, userdata) }
    }

    pub fn get_reduce_flashing(&self) -> bool {
        unsafe {
            let result = (*self.handle).getReduceFlashing.unwrap()();
            result == 1
        }
    }

    pub fn get_elapsed_time(&self) -> f32 {
        unsafe { (*self.handle).getElapsedTime.unwrap()() }
    }

    pub fn reset_elapsed_time(&self) {
        unsafe { (*self.handle).resetElapsedTime.unwrap()() }
    }

    pub fn get_battery_percentage(&self) -> f32 {
        unsafe { (*self.handle).getBatteryPercentage.unwrap()() }
    }

    pub fn get_battery_voltage(&self) -> f32 {
        unsafe { (*self.handle).getBatteryVoltage.unwrap()() }
    }

    pub fn get_timezone_offset(&self) -> i32 {
        unsafe { (*self.handle).getTimezoneOffset.unwrap()() }
    }

    pub fn should_display_24_hour_time(&self) -> bool {
        unsafe {
            let result = (*self.handle).shouldDisplay24HourTime.unwrap()();
            result == 1
        }
    }

    pub fn convert_epoch_to_date_time(&self, epoch: u32) -> DateTime {
        let mut datetime = DateTime::default();
        unsafe {
            (*self.handle).convertEpochToDateTime.unwrap()(epoch, &mut datetime);
            datetime
        }
    }

    pub fn convert_date_time_to_epoch(&self, mut datetime: DateTime) -> u32 {
        unsafe { (*self.handle).convertDateTimeToEpoch.unwrap()(&mut datetime) }
    }

    pub fn clear_icache(&self) {
        unsafe { (*self.handle).clearICache.unwrap()() }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct MenuItem {
    handle: *mut sys::PDMenuItem,
}

unsafe impl Send for MenuItem {}
unsafe impl Sync for MenuItem {}

impl MenuItem {
    fn new(handle: *mut sys::PDMenuItem) -> Self {
        MenuItem { handle }
    }

    pub fn get_value(&self) -> i32 {
        PLAYDATE.system.get_menu_item_value(self.handle)
    }

    pub fn set_value(&self, value: i32) {
        PLAYDATE.system.set_menu_item_value(self.handle, value)
    }

    pub fn get_title(&self) -> &str {
        let c_buf = PLAYDATE.system.get_menu_item_title(self.handle);
        let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
        let s: &str = c_str.to_str().unwrap();
        s
    }

    pub fn set_title(&self, title: impl AsRef<str>) {
        PLAYDATE.system.set_menu_item_title(self.handle, title)
    }

    pub fn get_userdata(&self) -> *mut c_void {
        PLAYDATE.system.get_menu_item_userdata(self.handle)
    }

    pub fn set_userdata(&self, userdata: *mut c_void) {
        PLAYDATE
            .system
            .set_menu_item_userdata(self.handle, userdata)
    }
}

impl Drop for MenuItem {
    fn drop(&mut self) {
        PLAYDATE.system.remove_menu_item(self.handle);
    }
}
