use core::ffi::{c_char, c_void, CStr};

use alloc::{ffi::CString, vec::Vec};
pub use sys::{
    PDButtons as Buttons, PDDateTime as DateTime, PDLanguage as Language,
    PDPeripherals as Peripherals, PDSystemEvent as SystemEvent,
};

use crate::{graphics::Bitmap, PLAYDATE};

pub struct System {
    handle: *const sys::playdate_sys,
}

impl System {
    pub(crate) fn new(handle: *const sys::playdate_sys) -> Self {
        System { handle }
    }

    /// Allocates heap space if ptr is NULL, else reallocates the given pointer. If size is zero, frees the given pointer.
    pub(crate) fn realloc(&self, ptr: *mut c_void, size: usize) -> *mut c_void {
        unsafe { (*self.handle).realloc.unwrap()(ptr, size) }
    }

    /// Calls the log function.
    pub fn log_to_console(&self, msg: impl AsRef<str>) {
        unsafe {
            let c_string = CString::new(msg.as_ref()).unwrap();
            (*self.handle).logToConsole.unwrap()(c_string.as_ptr() as *mut c_char);
        }
    }

    /// Calls the log function, outputting an error in red to the console, then pauses execution.
    pub fn error(&self, msg: impl AsRef<str>) {
        unsafe {
            let c_string = CString::new(msg.as_ref()).unwrap();
            (*self.handle).error.unwrap()(c_string.as_ptr() as *mut c_char);
        }
    }

    /// Returns the current language of the system.
    pub fn get_language(&self) -> Language {
        unsafe { (*self.handle).getLanguage.unwrap()() }
    }

    /// Returns the number of milliseconds since…​some arbitrary point in time. This should present a consistent timebase while a game is running, but the counter will be disabled when the device is sleeping.
    pub fn get_current_time_milliseconds(&self) -> usize {
        unsafe { (*self.handle).getCurrentTimeMilliseconds.unwrap()() as _ }
    }

    /// Returns the number of seconds (and sets milliseconds if not NULL) elapsed since midnight (hour 0), January 1, 2000.
    pub fn get_seconds_since_epoch(&self) -> (usize, usize) {
        let mut ms = 0;
        unsafe {
            let s = (*self.handle).getSecondsSinceEpoch.unwrap()(&mut ms);
            (s as _, ms as _)
        }
    }

    /// Calculates the current frames per second and draws that value at `x`, `y`.
    pub fn draw_fps(&self, x: i32, y: i32) {
        unsafe { (*self.handle).drawFPS.unwrap()(x, y) }
    }

    /// Replaces the default Lua run loop function with a custom update function. The update function should return a non-zero number to tell the system to update the display, or zero if update isn’t needed.
    pub(crate) fn set_update_callback(&self, update: sys::PDCallbackFunction) {
        unsafe {
            (*self.handle).setUpdateCallback.unwrap()(update, core::ptr::null_mut());
        }
    }

    /// Returns bitmasks indicating which buttons are currently down. pushed and released reflect which buttons were pushed or released over the previous update cycle—at the nominal frame rate of 50 ms, fast button presses can be missed if you just poll the instantaneous state.
    pub fn get_button_state(&self) -> ButtonState {
        let mut buttons = ButtonState {
            current: Buttons(0),
            pushed: Buttons(0),
            released: Buttons(0),
        };
        unsafe {
            (*self.handle).getButtonState.unwrap()(
                &mut buttons.current,
                &mut buttons.pushed,
                &mut buttons.released,
            );
            buttons
        }
    }

    /// By default, the accelerometer is disabled to save (a small amount of) power. To use a peripheral, it must first be enabled via this function. Accelerometer data is not available until the next update cycle after it’s enabled.
    pub fn set_peripherals_enabled(&self, mask: Peripherals) {
        unsafe {
            (*self.handle).setPeripheralsEnabled.unwrap()(mask);
        }
    }

    /// Returns the last-read accelerometer data.
    pub fn get_accelerometer(&self) -> (f32, f32, f32) {
        let x = core::ptr::null_mut();
        let y = core::ptr::null_mut();
        let z = core::ptr::null_mut();
        unsafe {
            (*self.handle).getAccelerometer.unwrap()(x, y, z);
            (*x, *y, *z)
        }
    }

    /// Returns the current position of the crank, in the range 0-360. Zero is pointing up, and the value increases as the crank moves clockwise, as viewed from the right side of the device.
    pub fn get_crank_angle(&self) -> f32 {
        unsafe { (*self.handle).getCrankAngle.unwrap()() }
    }

    /// Returns the angle change of the crank since the last time this function was called. Negative values are anti-clockwise.
    pub fn get_crank_change(&self) -> f32 {
        unsafe { (*self.handle).getCrankChange.unwrap()() }
    }

    /// Returns 1 or 0 indicating whether or not the crank is folded into the unit.
    pub fn is_crank_docked(&self) -> bool {
        unsafe {
            let result = (*self.handle).isCrankDocked.unwrap()();
            result == 1
        }
    }

    /// The function returns the previous value for this setting.
    pub fn set_crank_sounds_disabled(&self, flag: bool) -> bool {
        unsafe {
            let result = (*self.handle).setCrankSoundsDisabled.unwrap()(flag as i32);
            result == 1
        }
    }

    /// Returns 1 if the global "flipped" system setting is set, otherwise 0.
    pub fn get_flipped(&self) -> bool {
        unsafe {
            let result = (*self.handle).getFlipped.unwrap()();
            result == 1
        }
    }

    /// Disables or enables the 60 second auto lock feature. When called, the timer is reset to 60 seconds.
    pub fn set_auto_lock_disabled(&self, disable: bool) {
        unsafe { (*self.handle).setAutoLockDisabled.unwrap()(disable as i32) }
    }

    /// A game can optionally provide an image to be displayed alongside the system menu. bitmap must be a 400x240 LCDBitmap. All important content should be in the left half of the image in an area 200 pixels wide, as the menu will obscure the rest. The right side of the image will be visible briefly as the menu animates in and out.
    ///
    /// Optionally, a non-zero xoffset, can be provided. This must be a number between 0 and 200 and will cause the menu image to animate to a position offset left by xoffset pixels as the menu is animated in.
    ///
    /// This function could be called in response to the kEventPause event in your implementation of eventHandler().
    pub fn set_menu_image(&self, bitmap: &Bitmap, x_offset: i32) {
        unsafe { (*self.handle).setMenuImage.unwrap()(bitmap.handle, x_offset) }
    }

    /// title will be the title displayed by the menu item.
    ///
    /// Adds a new menu item to the System Menu. When invoked by the user, this menu item will:
    /// 1. Invoke your callback function.
    /// 2. Hide the System Menu.
    /// 3. Unpause your game and call eventHandler() with the kEventResume event.
    ///
    /// Your game can then present an options interface to the player, or take other action, in whatever manner you choose.
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

    /// Adds a new menu item that can be checked or unchecked by the player.
    ///
    /// title will be the title displayed by the menu item.
    ///
    /// value should be 0 for unchecked, 1 for checked.
    ///
    /// If this menu item is interacted with while the system menu is open, callback will be called when the menu is closed.
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

    /// Adds a new menu item that allows the player to cycle through a set of options.
    ///
    /// title will be the title displayed by the menu item.
    ///
    /// options should be an array of strings representing the states this menu item can cycle through. Due to limited horizontal space, the option strings and title should be kept short for this type of menu item.
    ///
    /// optionsCount should be the number of items contained in options.
    ///
    /// If this menu item is interacted with while the system menu is open, callback will be called when the menu is closed.
    pub fn add_options_menu_item(
        &self,
        title: impl AsRef<str>,
        option_titles: &[&str],
        options_count: i32,
        callback: sys::PDMenuItemCallbackFunction,
        userdata: *mut c_void,
    ) -> MenuItem {
        MenuItem::new(unsafe {
            let c_string = CString::new(title.as_ref()).unwrap();
            let title_cstrings = option_titles
                .iter()
                .map(|s| CString::new(*s).unwrap())
                .collect::<Vec<_>>();
            let mut title_ptrs = title_cstrings
                .iter()
                .map(|s| s.as_ptr() as *const c_char)
                .collect::<Vec<_>>();
            (*self.handle).addOptionsMenuItem.unwrap()(
                c_string.as_ptr() as *mut c_char,
                title_ptrs.as_mut_ptr(),
                options_count,
                callback,
                userdata,
            )
        })
    }

    /// Removes all custom menu items from the system menu.
    pub fn remove_all_menu_items(&self) {
        unsafe { (*self.handle).removeAllMenuItems.unwrap()() }
    }

    /// Removes the menu item from the system menu.
    pub(crate) fn remove_menu_item(&self, menu_item: *mut sys::PDMenuItem) {
        unsafe { (*self.handle).removeMenuItem.unwrap()(menu_item) }
    }

    /// Gets the integer value of the menu item.
    ///
    /// For checkmark menu items, 1 means checked, 0 unchecked. For option menu items, the value indicates the array index of the currently selected option.
    pub(crate) fn get_menu_item_value(&self, menu_item: *mut sys::PDMenuItem) -> i32 {
        unsafe { (*self.handle).getMenuItemValue.unwrap()(menu_item) }
    }

    /// Sets the integer value of the menu item.
    ///
    /// For checkmark menu items, 1 means checked, 0 unchecked. For option menu items, the value indicates the array index of the currently selected option.
    pub(crate) fn set_menu_item_value(&self, menu_item: *mut sys::PDMenuItem, value: i32) {
        unsafe { (*self.handle).setMenuItemValue.unwrap()(menu_item, value) }
    }

    /// Gets the display title of the menu item.
    pub(crate) fn get_menu_item_title(&self, menu_item: *mut sys::PDMenuItem) -> *const c_char {
        unsafe { (*self.handle).getMenuItemTitle.unwrap()(menu_item) }
    }

    /// Sets the display title of the menu item.
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

    /// Gets the userdata value associated with this menu item.
    pub(crate) fn get_menu_item_userdata(&self, menu_item: *mut sys::PDMenuItem) -> *mut c_void {
        unsafe { (*self.handle).getMenuItemUserdata.unwrap()(menu_item) }
    }

    /// Sets the userdata value associated with this menu item.
    pub(crate) fn set_menu_item_userdata(
        &self,
        menu_item: *mut sys::PDMenuItem,
        userdata: *mut c_void,
    ) {
        unsafe { (*self.handle).setMenuItemUserdata.unwrap()(menu_item, userdata) }
    }

    /// Returns 1 if the global "reduce flashing" system setting is set, otherwise 0.
    pub fn get_reduce_flashing(&self) -> bool {
        unsafe {
            let result = (*self.handle).getReduceFlashing.unwrap()();
            result == 1
        }
    }

    /// Returns the number of seconds since playdate.resetElapsedTime() was called. The value is a floating-point number with microsecond accuracy.
    pub fn get_elapsed_time(&self) -> f32 {
        unsafe { (*self.handle).getElapsedTime.unwrap()() }
    }

    /// Resets the high-resolution timer.
    pub fn reset_elapsed_time(&self) {
        unsafe { (*self.handle).resetElapsedTime.unwrap()() }
    }

    /// Returns a value from 0-100 denoting the current level of battery charge. 0 = empty; 100 = full.
    pub fn get_battery_percentage(&self) -> f32 {
        unsafe { (*self.handle).getBatteryPercentage.unwrap()() }
    }

    /// Returns the battery’s current voltage level.
    pub fn get_battery_voltage(&self) -> f32 {
        unsafe { (*self.handle).getBatteryVoltage.unwrap()() }
    }

    /// Returns the system timezone offset from GMT, in seconds.
    pub fn get_timezone_offset(&self) -> i32 {
        unsafe { (*self.handle).getTimezoneOffset.unwrap()() }
    }

    /// Returns 1 if the user has set the 24-Hour Time preference in the Settings program.
    pub fn should_display_24_hour_time(&self) -> bool {
        unsafe {
            let result = (*self.handle).shouldDisplay24HourTime.unwrap()();
            result == 1
        }
    }

    /// Converts the given epoch time to a PDDateTime.
    pub fn convert_epoch_to_date_time(&self, epoch: u32) -> DateTime {
        let mut datetime = DateTime::default();
        unsafe {
            (*self.handle).convertEpochToDateTime.unwrap()(epoch, &mut datetime);
            datetime
        }
    }

    /// Converts the given PDDateTime to an epoch time.
    pub fn convert_date_time_to_epoch(&self, mut datetime: DateTime) -> u32 {
        unsafe { (*self.handle).convertDateTimeToEpoch.unwrap()(&mut datetime) }
    }

    /// Flush the CPU instruction cache, on the very unlikely chance you’re modifying instruction code on the fly. (If you don’t know what I’m talking about, you don’t need this. :smile:)
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

    /// Gets the integer value of the menu item.
    ///
    /// For checkmark menu items, 1 means checked, 0 unchecked. For option menu items, the value indicates the array index of the currently selected option.
    pub fn get_value(&self) -> i32 {
        PLAYDATE.system.get_menu_item_value(self.handle)
    }

    /// Sets the integer value of the menu item.
    ///
    /// For checkmark menu items, 1 means checked, 0 unchecked. For option menu items, the value indicates the array index of the currently selected option.
    pub fn set_value(&self, value: i32) {
        PLAYDATE.system.set_menu_item_value(self.handle, value)
    }

    /// Gets the display title of the menu item.
    pub fn get_title(&self) -> &str {
        let c_buf = PLAYDATE.system.get_menu_item_title(self.handle);
        let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
        let s: &str = c_str.to_str().unwrap();
        s
    }

    /// Sets the display title of the menu item.
    pub fn set_title(&self, title: impl AsRef<str>) {
        PLAYDATE.system.set_menu_item_title(self.handle, title)
    }

    /// Gets the userdata value associated with this menu item.
    pub fn get_userdata(&self) -> *mut c_void {
        PLAYDATE.system.get_menu_item_userdata(self.handle)
    }

    /// Sets the userdata value associated with this menu item.
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

#[derive(Debug)]
pub struct ButtonState {
    pub current: Buttons,
    pub pushed: Buttons,
    pub released: Buttons,
}
