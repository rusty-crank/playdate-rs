#![no_std]
// #![feature(global_allocator, allocator_api, heap_api)]

extern crate alloc;
pub extern crate playdate_rs_sys as sys;

pub mod graphics;
mod memory;
#[macro_use]
pub mod print;
pub mod system;
pub mod video;

pub use playdate_rs_macros::app;

pub struct Playdate {
    pub system: system::System,
    // pub file: *const playdate_file,
    pub graphics: graphics::Graphics,
    // pub sprite: *const playdate_sprite,
    // pub display: *const playdate_display,
    // pub sound: *const playdate_sound,
    // pub lua: *const playdate_lua,
    // pub json: *const playdate_json,
    // pub scoreboards: *const playdate_scoreboards,
}

unsafe impl Sync for Playdate {}
unsafe impl Send for Playdate {}

impl Playdate {
    fn new(playdate: *mut playdate_rs_sys::PlaydateAPI) -> Self {
        let playdate_ref = unsafe { &*playdate };
        Self {
            system: system::System::new(playdate_ref.system),
            graphics: graphics::Graphics::new(playdate_ref.graphics),
        }
    }
}

static INIT: spin::Once = spin::Once::new();

static mut PLAYDATE_PTR: *mut playdate_rs_sys::PlaydateAPI = core::ptr::null_mut();

pub static PLAYDATE: spin::Lazy<Playdate> =
    spin::Lazy::new(|| Playdate::new(unsafe { PLAYDATE_PTR }));

pub trait App {
    fn init(&self) {}
    fn update(&self) {}
    fn handle_event(&self, _event: system::SystemEvent, _arg: u32) {}
}

pub fn init_playdate_once(pd: *mut playdate_rs_sys::PlaydateAPI) {
    INIT.call_once(|| unsafe {
        PLAYDATE_PTR = pd;
        spin::Lazy::force(&PLAYDATE);
    });
}

static mut APP: Option<&'static dyn App> = None;

extern "C" fn update(_: *mut core::ffi::c_void) -> i32 {
    unsafe {
        APP.as_ref().unwrap().update();
    }
    1
}

pub fn start(app: &'static dyn App) {
    unsafe {
        APP = Some(app);
    }
    app.init();
    PLAYDATE.system.set_update_callback(Some(update));
}

#[macro_export]
macro_rules! register_playdate_app {
    ($app: ident) => {
        #[no_mangle]
        unsafe extern "C" fn eventHandler(
            pd: *mut $crate::sys::PlaydateAPI,
            event: $crate::system::SystemEvent,
            arg: u32,
        ) {
            if event == $crate::system::SystemEvent::kEventInit {
                $crate::init_playdate_once(pd);
                $crate::start(&$app);
            }
            $app.handle_event(event, arg);
        }
    };
}
