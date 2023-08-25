#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]

extern crate alloc;
pub extern crate playdate_rs_sys as sys;

#[macro_use]
#[doc(hidden)]
pub mod print;

pub mod display;
pub mod error;
pub mod fs;
pub mod graphics;
pub mod lua;
pub mod math;
mod memory;
pub mod scoreboards;
pub mod sound;
pub mod sprite;
pub mod system;
pub mod util;
pub mod video;

use alloc::{boxed::Box, format};
pub use no_std_io::io;
pub use playdate_rs_macros::app;

pub struct Playdate {
    pub raw_api: *mut sys::PlaydateAPI,
    pub system: system::System,
    pub file: fs::FileSystem,
    pub graphics: graphics::Graphics,
    pub sprite: sprite::_Sprite,
    pub display: display::Display,
    pub sound: sound::Sound,
    pub scoreboards: scoreboards::Scoreboards,
    pub lua: lua::Lua,
    // The playdate JSON lib is not supported. Please use serde instead:
    // pub json: *const playdate_json,
}

unsafe impl Sync for Playdate {}
unsafe impl Send for Playdate {}

impl Playdate {
    fn new(playdate: *mut sys::PlaydateAPI) -> Self {
        let playdate_ref = unsafe { &*playdate };
        Self {
            raw_api: playdate,
            system: system::System::new(playdate_ref.system),
            file: fs::FileSystem::new(playdate_ref.file),
            graphics: graphics::Graphics::new(playdate_ref.graphics),
            sprite: sprite::_Sprite::new(playdate_ref.sprite),
            display: display::Display::new(playdate_ref.display),
            sound: sound::Sound::new(playdate_ref.sound),
            scoreboards: scoreboards::Scoreboards::new(playdate_ref.scoreboards),
            lua: lua::Lua::new(playdate_ref.lua),
        }
    }
}

static INIT: spin::Once = spin::Once::new();

static mut PLAYDATE_PTR: *mut sys::PlaydateAPI = core::ptr::null_mut();

pub static PLAYDATE: spin::Lazy<Playdate> =
    spin::Lazy::new(|| Playdate::new(unsafe { PLAYDATE_PTR }));

pub trait App: Sized + 'static {
    /// Constructor for the app. This is called once when the app is loaded.
    fn new() -> Self;

    /// Returns a reference to the app singleton.
    fn get() -> &'static mut Self {
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
    let app = T::get();
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
    INIT.call_once(|| unsafe {
        PLAYDATE_PTR = pd;
        spin::Lazy::force(&PLAYDATE);
    });
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
    pd: *mut sys::PlaydateAPI,
    event: system::SystemEvent,
    arg: u32,
) {
    if event == system::SystemEvent::kEventInit {
        start_playdate_app::<T>(pd);
    }
    T::get().handle_event(event, arg);
}

#[doc(hidden)]
pub fn __playdate_handle_panic(info: &core::panic::PanicInfo) -> ! {
    PLAYDATE.system.error(format!("{}", info));
    loop {
        unreachable!()
    }
}

#[macro_export]
macro_rules! register_playdate_app {
    ($app: ident) => {
        mod __playdate_api {
            #[no_mangle]
            unsafe extern "C" fn eventHandler(
                pd: *mut $crate::sys::PlaydateAPI,
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
