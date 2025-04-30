#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![feature(arbitrary_self_types)]

extern crate alloc;
pub extern crate num_traits;
#[doc(hidden)]
pub extern crate playdate_rs_sys as sys;
pub extern crate rand;
pub extern crate serde;
pub extern crate serde_json;

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
pub mod network;
pub mod scoreboards;
pub mod sound;
pub mod sprite;
pub mod system;
pub mod util;

#[doc(hidden)]
pub mod async_runtime;

use core::{cell::UnsafeCell, future::Future, ops::Deref};

use alloc::format;
use async_runtime::EXECUTOR;
pub use no_std_io::io;
pub use playdate_rs_macros::main;
use system::EventManager;

pub struct PlaydateAPI {
    raw_api: *mut sys::PlaydateAPI,
    /// System interaction
    pub system: system::PlaydateSystem,
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
    pub events: EventManager,
}

unsafe impl Sync for PlaydateAPI {}
unsafe impl Send for PlaydateAPI {}

impl PlaydateAPI {
    fn new(playdate: *mut sys::PlaydateAPI) -> Self {
        let playdate_ref = unsafe { &*playdate };
        Self {
            raw_api: playdate,
            system: system::PlaydateSystem::new(playdate_ref.system),
            graphics: graphics::PlaydateGraphics::new(playdate_ref.graphics),
            sprite: sprite::PlaydateSprite::new(playdate_ref.sprite),
            display: display::PlaydateDisplay::new(playdate_ref.display),
            sound: sound::PlaydateSound::new(playdate_ref.sound),
            scoreboards: scoreboards::PlaydateScoreboards::new(playdate_ref.scoreboards),
            lua: lua::Lua::new(playdate_ref.lua),
            events: EventManager::new(),
        }
    }

    /// Returns a raw pointer to the raw playdate-rs-sys API.
    pub fn get_raw_api(&self) -> *mut sys::PlaydateAPI {
        self.raw_api
    }

    pub fn next_frame(&self) -> impl Future<Output = f32> {
        EXECUTOR.next_frame()
    }

    pub fn spawn(&self, future: impl 'static + Future<Output = ()>) {
        EXECUTOR.spawn(future);
    }

    pub fn sleep(&self, ms: usize) -> impl Future<Output = ()> {
        EXECUTOR.sleep(ms)
    }

    pub fn yield_now(&self) -> impl Future<Output = ()> {
        EXECUTOR.yield_now()
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

#[macro_export]
macro_rules! register_playdate_app {
    ($main: ident) => {
        mod __playdate_api {
            #[no_mangle]
            unsafe extern "C" fn eventHandler(
                pd: *mut ::core::ffi::c_void,
                event: $crate::system::SystemEvent,
                arg: u32,
            ) {
                $crate::__playdate_handle_event(pd, event, arg, super::$main);
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
        extern "C" fn _sbrk() {}

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

#[doc(hidden)]
pub fn __playdate_handle_panic(info: &core::panic::PanicInfo) -> ! {
    PLAYDATE.system.error(format!("{info}"));
    unreachable!()
}

#[doc(hidden)]
pub fn __playdate_handle_event<F: 'static + Future<Output = ()>>(
    pd: *mut ::core::ffi::c_void,
    event: system::SystemEvent,
    arg: u32,
    main: fn() -> F,
) {
    let pd = pd as *mut sys::PlaydateAPI;
    if event == system::SystemEvent::Init {
        // Initialize playdate singleton
        unsafe {
            *PLAYDATE._p.get() = Some(PlaydateAPI::new(pd));
        }
        // Register frame update callback
        PLAYDATE
            .system
            .set_update_callback(Some(handle_frame_update));
        // Run the main function
        EXECUTOR.spawn(main());
        EXECUTOR.run();
    }
    PLAYDATE.events.signal(event, arg);
}

unsafe extern "C" fn handle_frame_update(_: *mut core::ffi::c_void) -> i32 {
    EXECUTOR.signal_next_frame();
    if PLAYDATE.display.should_update() {
        1
    } else {
        0
    }
}

#[macro_export]
macro_rules! spawn {
    ($($body:tt)*) => {
        $crate::PLAYDATE.spawn(async move {
            $($body)*
        });
    };
}
