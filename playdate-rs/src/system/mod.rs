use core::ffi::{c_char, c_void, CStr};

use alloc::{boxed::Box, ffi::CString, vec::Vec};
pub use sys::{
    LCDFontData as FontData, PDDateTime as DateTime, PDLanguage as Language,
    PDSystemEvent as SystemEvent,
};
use sys::{PDButtons, PDPeripherals};

use crate::{graphics::Bitmap, math::Vec2};

mod events;
mod menu;

pub use events::EventManager;
pub use menu::MenuItem;

pub struct PlaydateSystem {
    handle: *const sys::playdate_sys,
}

impl PlaydateSystem {
    pub(crate) fn new(handle: *const sys::playdate_sys) -> Self {
        Self { handle }
    }

    /// Allocates heap space if ptr is NULL, else reallocates the given pointer. If size is zero, frees the given pointer.
    pub(crate) fn realloc(&self, ptr: *mut c_void, size: usize) -> *mut c_void {
        unsafe { (*self.handle).realloc.unwrap()(ptr, size) }
    }

    /// Calls the log function.
    pub(crate) fn log_to_console(&self, msg: impl AsRef<str>) {
        unsafe {
            let c_string = CString::new(msg.as_ref()).unwrap();
            (*self.handle).logToConsole.unwrap()(c_string.as_ptr() as *mut c_char);
        }
    }

    /// Calls the log function, outputting an error in red to the console, then pauses execution.
    pub(crate) fn error(&self, msg: impl AsRef<str>) {
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
    pub fn draw_fps(&self, pos: Vec2<i32>) {
        unsafe { (*self.handle).drawFPS.unwrap()(pos.x, pos.y) }
    }

    /// Replaces the default Lua run loop function with a custom update function. The update function should return a non-zero number to tell the system to update the display, or zero if update isn’t needed.
    pub(crate) fn set_update_callback(&self, update: sys::PDCallbackFunction) {
        unsafe {
            (*self.handle).setUpdateCallback.unwrap()(update, core::ptr::null_mut());
        }
    }

    /// Returns bitmasks indicating which buttons are currently down. pushed and released reflect which buttons were pushed or released over the previous update cycle—at the nominal frame rate of 50 ms, fast button presses can be missed if you just poll the instantaneous state.
    pub fn get_button_state(&self) -> ButtonState {
        let mut current = PDButtons(0);
        let mut pushed = PDButtons(0);
        let mut released = PDButtons(0);
        unsafe {
            (*self.handle).getButtonState.unwrap()(&mut current, &mut pushed, &mut released);
        }
        ButtonState {
            current: Buttons::from(current.0 as u8),
            pushed: Buttons::from(pushed.0 as u8),
            released: Buttons::from(released.0 as u8),
        }
    }

    /// By default, the accelerometer is disabled to save (a small amount of) power. To use a peripheral, it must first be enabled via this function. Accelerometer data is not available until the next update cycle after it’s enabled.
    pub fn set_peripherals_enabled(&self, mask: Peripherals) {
        unsafe {
            (*self.handle).setPeripheralsEnabled.unwrap()(PDPeripherals(mask.bits() as _));
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
    pub fn set_menu_image(&self, bitmap: impl AsRef<Bitmap>, x_offset: i32) {
        unsafe { (*self.handle).setMenuImage.unwrap()(bitmap.as_ref().handle, x_offset) }
    }

    /// title will be the title displayed by the menu item.
    ///
    /// Adds a new menu item to the System Menu. When invoked by the user, this menu item will:
    /// 1. Invoke your callback function.
    /// 2. Hide the System Menu.
    /// 3. Unpause your game and call eventHandler() with the kEventResume event.
    ///
    /// Your game can then present an options interface to the player, or take other action, in whatever manner you choose.
    pub fn add_menu_item(&self, title: impl AsRef<str>) -> MenuItem {
        let mut menu_item = MenuItem::new();
        let handle = unsafe {
            let c_string = CString::new(title.as_ref()).unwrap();
            (*self.handle).addMenuItem.unwrap()(
                c_string.as_ptr() as *mut c_char,
                Some(MenuItem::callback),
                menu_item.payload_ptr(),
            )
        };
        menu_item.set_handle(handle);
        menu_item
    }

    /// Adds a new menu item that can be checked or unchecked by the player.
    ///
    /// title will be the title displayed by the menu item.
    ///
    /// value should be false for unchecked, true for checked.
    ///
    /// If this menu item is interacted with while the system menu is open, callback will be called when the menu is closed.
    pub fn add_checkmark_menu_item(&self, title: impl AsRef<str>, value: bool) -> MenuItem {
        let mut menu_item = MenuItem::new();
        let handle = unsafe {
            let c_string = CString::new(title.as_ref()).unwrap();
            (*self.handle).addCheckmarkMenuItem.unwrap()(
                c_string.as_ptr() as *mut c_char,
                value as _,
                Some(MenuItem::callback),
                menu_item.payload_ptr(),
            )
        };
        menu_item.set_handle(handle);
        menu_item
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
    ) -> MenuItem {
        let mut menu_item = MenuItem::new();
        let handle = unsafe {
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
                option_titles.len() as _,
                Some(MenuItem::callback),
                menu_item.payload_ptr(),
            )
        };
        menu_item.set_handle(handle);
        menu_item
    }

    /// Removes all custom menu items from the system menu.
    #[allow(unused)]
    pub(crate) fn remove_all_menu_items(&self) {
        unsafe { (*self.handle).removeAllMenuItems.unwrap()() }
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

    /// (2.4) As an alternative to polling for button presses using getButtonState(), this function allows a callback function to be set. The function is called for each button up/down event (possibly multiple events on the same button) that occurred during the previous update cycle. At the default 30 FPS, a queue size of 5 should be adequate. At lower frame rates/longer frame times, the queue size should be extended until all button presses are caught. The function should return 0 on success or a non-zero value to signal an error.
    #[allow(static_mut_refs)]
    pub fn set_button_callback(
        &self,
        callback: Option<Box<dyn FnMut(Buttons, i32, u32)>>,
        queue_size: usize,
    ) {
        static mut CALLBACK: Option<Box<dyn FnMut(Buttons, i32, u32)>> = None;
        extern "C" fn callback_impl(
            buttons: PDButtons,
            down: i32,
            when: u32,
            _userdata: *mut core::ffi::c_void,
        ) -> i32 {
            let callback = unsafe { CALLBACK.as_mut().unwrap() };
            callback(Buttons::from(buttons.0 as u8), down, when);
            0
        }
        unsafe {
            let callback_exists = callback.is_some();
            CALLBACK = callback;
            ((*self.handle).setButtonCallback.unwrap())(
                if callback_exists {
                    Some(callback_impl)
                } else {
                    None
                },
                core::ptr::null_mut(),
                queue_size as _,
            );
        }
    }

    /// (2.4) Provides a callback to receive messages sent to the device over the serial port using the msg command. If no device is connected, you can send these messages to a game in the simulator by entering !msg <message> in the Lua console.
    #[allow(static_mut_refs)]
    pub fn set_serial_message_callback(&self, callback: Option<Box<dyn FnMut(&[u8])>>) {
        static mut CALLBACK: Option<Box<dyn FnMut(&[u8])>> = None;
        extern "C" fn callback_impl(data: *const c_char) {
            let callback = unsafe { CALLBACK.as_mut().unwrap() };
            let c_str: &CStr = unsafe { CStr::from_ptr(data) };
            let s: &[u8] = c_str.to_bytes();
            callback(s);
        }
        unsafe {
            let callback_exists = callback.is_some();
            CALLBACK = callback;
            ((*self.handle).setSerialMessageCallback.unwrap())(if callback_exists {
                Some(callback_impl)
            } else {
                None
            });
        }
    }

    /// (???) Pauses execution for the given number of milliseconds.
    pub fn delay(&self, ms: usize) {
        unsafe { (*self.handle).delay.unwrap()(ms as _) }
    }
}

#[derive(Debug)]
pub struct ButtonState {
    pub current: Buttons,
    pub pushed: Buttons,
    pub released: Buttons,
}

#[bitmask_enum::bitmask(u8)]
pub enum Buttons {
    Left = 1 << 0,
    Right = 1 << 1,
    Up = 1 << 2,
    Down = 1 << 3,
    B = 1 << 4,
    A = 1 << 5,
}

#[bitmask_enum::bitmask(u16)]
pub enum Peripherals {
    Accelerometer = 1 << 0,
}

impl Peripherals {
    pub const NONE: Self = Peripherals::none();
    pub const ALL: Self = Peripherals::all_bits();
}
