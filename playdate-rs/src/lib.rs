#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]

extern crate alloc;
pub extern crate num_traits;
#[doc(hidden)]
pub extern crate playdate_rs_sys as sys;
pub extern crate rand;

#[macro_use]
#[doc(hidden)]
pub mod print;

#[macro_use]
pub mod math;

pub mod display;
pub mod error;
pub mod fs;
pub mod graphics;
pub mod lua;
mod memory;
pub mod scoreboards;
pub mod sound;
pub mod sprite;
pub mod system;
pub mod util;
pub mod video;

use core::{cell::UnsafeCell, ops::Deref};

use alloc::{boxed::Box, format};
pub use no_std_io::io;
pub use playdate_rs_macros::app;

pub struct PlaydateAPI {
    raw_api: *mut sys::PlaydateAPI,
    /// System interaction
    pub system: system::PlaydateSystem,
    /// Filesystem operations
    pub file: fs::PlaydateFileSystem,
    /// Graphics operations and drawing functions
    pub graphics: graphics::PlaydateGraphics,
    /// Sprite and global sprite display list operations
    pub sprite: sprite::PlaydateSprite,
    /// Display operations and management
    pub display: display::PlaydateDisplay,
    /// Sound controls
    pub sound: sound::PlaydateSound,
    /// Scoreboard operations (unimplemented)
    pub scoreboards: scoreboards::PlaydateScoreboards,
    /// Lua VM interactions (unimplemented)
    pub lua: lua::Lua,
    // The playdate JSON lib is not supported. Please use serde instead:
    // pub json: *const playdate_json,
}

unsafe impl Sync for PlaydateAPI {}
unsafe impl Send for PlaydateAPI {}

impl PlaydateAPI {
    fn new(playdate: *mut sys::PlaydateAPI) -> Self {
        let playdate_ref = unsafe { &*playdate };
        Self {
            raw_api: playdate,
            system: system::PlaydateSystem::new(playdate_ref.system),
            file: fs::PlaydateFileSystem::new(playdate_ref.file),
            graphics: graphics::PlaydateGraphics::new(playdate_ref.graphics),
            sprite: sprite::PlaydateSprite::new(playdate_ref.sprite),
            display: display::PlaydateDisplay::new(playdate_ref.display),
            sound: sound::PlaydateSound::new(playdate_ref.sound),
            scoreboards: scoreboards::PlaydateScoreboards::new(playdate_ref.scoreboards),
            lua: lua::Lua::new(playdate_ref.lua),
        }
    }

    /// Returns a raw pointer to the raw playdate-rs-sys API.
    pub fn get_raw_api(&self) -> *mut sys::PlaydateAPI {
        self.raw_api
    }
}

pub static PLAYDATE: Playdate = Playdate {
    _p: UnsafeCell::new(None),
};

pub struct Playdate {
    _p: UnsafeCell<Option<PlaydateAPI>>,
}

unsafe impl Sync for Playdate {}
unsafe impl Send for Playdate {}

impl Deref for Playdate {
    type Target = PlaydateAPI;

    fn deref(&self) -> &Self::Target {
        unsafe { (*self._p.get()).as_ref().unwrap() }
    }
}

pub trait App: Sized + 'static {
    /// Constructor for the app. This is called once when the app is loaded.
    fn new() -> Self;

    /// Returns a reference to the app singleton.
    fn get() -> &'static Self {
        unsafe { &*(APP.unwrap() as *const Self) }
    }

    /// # Safety
    ///
    /// Should always be safe as this is a single-threaded environment.
    unsafe fn get_mut() -> &'static mut Self {
        unsafe { &mut *(APP.unwrap() as *mut Self) }
    }

    /// Called once when the app is loaded.
    fn init(&mut self) {}

    /// Called once per frame.
    ///
    /// `delta` is the time in seconds since the last frame.
    fn update(&mut self, _delta: f32) {}

    /// Called when a system event occurs.
    fn handle_event(&mut self, _event: system::SystemEvent, _arg: u32) {}
}

static mut APP: Option<*mut ()> = None;

unsafe extern "C" fn update<T: App>(_: *mut core::ffi::c_void) -> i32 {
    let app = T::get_mut();
    // calculate delta time since last frame
    let delta_time = {
        static mut LAST_FRAME_TIME: Option<usize> = None;
        let current_time = PLAYDATE.system.get_current_time_milliseconds();
        let delta = if let Some(last_frame_time) = LAST_FRAME_TIME {
            (current_time - last_frame_time) as f32 / 1000.0
        } else {
            0.0
        };
        LAST_FRAME_TIME = Some(current_time);
        delta
    };
    // update frame
    app.update(delta_time);
    1
}

fn start_playdate_app<T: App>(pd: *mut sys::PlaydateAPI) {
    // Initialize playdate singleton
    unsafe {
        *PLAYDATE._p.get() = Some(PlaydateAPI::new(pd));
    }
    // Create app instance
    let app = Box::leak(Box::new(T::new()));
    unsafe {
        APP = Some(app as *mut T as *mut ());
    }
    // Initialize app
    app.init();
    PLAYDATE.system.set_update_callback(Some(update::<T>));
}

#[doc(hidden)]
pub fn __playdate_handle_event<T: App>(
    pd: *mut ::core::ffi::c_void,
    event: system::SystemEvent,
    arg: u32,
) {
    let pd = pd as *mut sys::PlaydateAPI;
    if event == system::SystemEvent::Init {
        start_playdate_app::<T>(pd);
    }
    unsafe { T::get_mut().handle_event(event, arg) };
}

#[doc(hidden)]
pub fn __playdate_handle_panic(info: &core::panic::PanicInfo) -> ! {
    PLAYDATE.system.error(format!("{}", info));
    unreachable!()
}

#[macro_export]
macro_rules! register_playdate_app {
    ($app: ident) => {
        mod __playdate_api {
            #[no_mangle]
            unsafe extern "C" fn eventHandler(
                pd: *mut ::core::ffi::c_void,
                event: $crate::system::SystemEvent,
                arg: u32,
            ) {
                $crate::__playdate_handle_event::<super::$app>(pd, event, arg);
            }
        }

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        #[panic_handler]
        #[doc(hidden)]
        fn __panic_handler(info: &core::panic::PanicInfo) -> ! {
            $crate::__playdate_handle_panic(info);
        }

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        #[no_mangle]
        pub extern "C" fn _sbrk() {}

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        #[no_mangle]
        extern "C" fn _exit() {}

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        #[no_mangle]
        extern "C" fn _kill() {}

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        #[no_mangle]
        extern "C" fn _getpid() {}

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        #[no_mangle]
        extern "C" fn __exidx_start() {
            unimplemented!();
        }

        #[cfg(all(target_arch = "arm", target_os = "none"))]
        #[no_mangle]
        extern "C" fn __exidx_end() {
            unimplemented!();
        }
    };
}
