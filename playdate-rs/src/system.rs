use core::ffi::{c_char, c_void};

use alloc::ffi::CString;
pub use sys::{
    PDButtons as Buttons, PDLanguage as Language, PDMenuItem as MenuItem,
    PDPeripherals as Peripherals, PDSystemEvent as SystemEvent,
};

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

    // unsafe extern "C" fn(ret: *mut *mut ::core::ffi::c_char, fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int>

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

    //     pub setMenuImage: ::core::option::Option<
    //     unsafe extern "C" fn(bitmap: *mut LCDBitmap, xOffset: ::core::ffi::c_int),
    // >,

    pub fn add_menu_item(
        &self,
        title: impl AsRef<str>,
        callback: sys::PDMenuItemCallbackFunction,
        userdata: *mut c_void,
    ) -> *mut MenuItem {
        unsafe {
            let c_string = CString::new(title.as_ref()).unwrap();
            (*self.handle).addMenuItem.unwrap()(
                c_string.as_ptr() as *mut c_char,
                callback,
                userdata,
            )
        }
    }

    pub fn add_checkmark_menu_item(
        &self,
        title: impl AsRef<str>,
        value: i32,
        callback: sys::PDMenuItemCallbackFunction,
        userdata: *mut c_void,
    ) -> *mut MenuItem {
        unsafe {
            let c_string = CString::new(title.as_ref()).unwrap();
            (*self.handle).addCheckmarkMenuItem.unwrap()(
                c_string.as_ptr() as *mut c_char,
                value,
                callback,
                userdata,
            )
        }
    }

    pub fn add_options_menu_item(
        &self,
        title: impl AsRef<str>,
        option_titles: *mut *const c_char,
        options_count: i32,
        callback: sys::PDMenuItemCallbackFunction,
        userdata: *mut c_void,
    ) -> *mut MenuItem {
        unsafe {
            let c_string = CString::new(title.as_ref()).unwrap();
            (*self.handle).addOptionsMenuItem.unwrap()(
                c_string.as_ptr() as *mut c_char,
                option_titles,
                options_count,
                callback,
                userdata,
            )
        }
    }

    pub fn remove_all_menu_items(&self) {
        unsafe { (*self.handle).removeAllMenuItems.unwrap()() }
    }

    pub fn remove_menu_item(&self, menu_item: *mut MenuItem) {
        unsafe { (*self.handle).removeMenuItem.unwrap()(menu_item) }
    }

    pub fn get_menu_item_value(&self, menu_item: *mut MenuItem) -> i32 {
        unsafe { (*self.handle).getMenuItemValue.unwrap()(menu_item) }
    }

    pub fn set_menu_item_value(&self, menu_item: *mut MenuItem, value: i32) {
        unsafe { (*self.handle).setMenuItemValue.unwrap()(menu_item, value) }
    }

    pub fn get_menu_item_title(&self, menu_item: *mut MenuItem) -> *const c_char {
        unsafe { (*self.handle).getMenuItemTitle.unwrap()(menu_item) }
    }

    pub fn set_menu_item_title(&self, menu_item: *mut MenuItem, title: impl AsRef<str>) {
        unsafe {
            let c_string = CString::new(title.as_ref()).unwrap();
            (*self.handle).setMenuItemTitle.unwrap()(menu_item, c_string.as_ptr() as *mut c_char)
        }
    }

    pub fn get_menu_item_userdata(&self, menu_item: *mut MenuItem) -> *mut c_void {
        unsafe { (*self.handle).getMenuItemUserdata.unwrap()(menu_item) }
    }

    pub fn set_menu_item_userdata(&self, menu_item: *mut MenuItem, userdata: *mut c_void) {
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

    pub fn convert_epoch_to_date_time(&self, epoch: u32) -> sys::PDDateTime {
        let mut datetime = sys::PDDateTime::default();
        unsafe {
            (*self.handle).convertEpochToDateTime.unwrap()(epoch, &mut datetime);
            datetime
        }
    }

    pub fn convert_date_time_to_epoch(&self, mut datetime: sys::PDDateTime) -> u32 {
        unsafe { (*self.handle).convertDateTimeToEpoch.unwrap()(&mut datetime) }
    }

    pub fn clear_icache(&self) {
        unsafe { (*self.handle).clearICache.unwrap()() }
    }
}
