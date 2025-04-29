use alloc::boxed::Box;
use core::cell::RefCell;
use core::ffi::c_void;

use alloc::sync::Arc;

use crate::PLAYDATE;

pub(crate) struct PlaydateSoundSource {
    pub(crate) handle: *const sys::playdate_sound_source,
}

impl PlaydateSoundSource {
    pub(crate) fn new(handle: *const sys::playdate_sound_source) -> Self {
        Self { handle }
    }
}

struct Callbacks {
    finish: Option<Box<dyn FnMut()>>,
}

pub struct SoundSource {
    pub(crate) handle: *mut sys::SoundSource,
    callbacks: Arc<RefCell<Callbacks>>,
}

unsafe impl Send for SoundSource {}
unsafe impl Sync for SoundSource {}

impl SoundSource {
    pub(crate) fn new(handle: *mut sys::SoundSource) -> Self {
        Self {
            handle,
            callbacks: Arc::new(RefCell::new(Callbacks { finish: None })),
        }
    }

    /// Sets the playback volume (0.0 - 1.0) for left and right channels of the source.
    pub fn set_volume(&self, left: f32, right: f32) {
        unsafe { (*PLAYDATE.sound.source.handle).setVolume.unwrap()(self.handle, left, right) };
    }

    /// Gets the playback volume (0.0 - 1.0) for left and right channels of the source.
    pub fn get_volume(&self) -> (f32, f32) {
        let mut left = 0.0;
        let mut right = 0.0;
        unsafe {
            (*PLAYDATE.sound.source.handle).getVolume.unwrap()(self.handle, &mut left, &mut right)
        };
        (left, right)
    }

    /// Returns true if the source is currently playing.
    pub fn is_playing(&self) -> bool {
        unsafe { (*PLAYDATE.sound.source.handle).isPlaying.unwrap()(self.handle) == 1 }
    }

    pub(crate) fn set_finish_callback(&self, callback: impl FnMut() + 'static) {
        self.callbacks.borrow_mut().finish = Some(Box::new(callback));
        let callbacks = self.callbacks.as_ptr() as *const RefCell<Callbacks>;
        extern "C" fn callback_fn(_source: *mut sys::SoundSource, userdata: *mut c_void) {
            let callback = unsafe { &*(userdata as *const RefCell<Callbacks>) };
            if let Some(ref mut f) = callback.borrow_mut().finish {
                f();
            }
        }
        unsafe {
            (*PLAYDATE.sound.source.handle).setFinishCallback.unwrap()(
                self.handle,
                Some(callback_fn),
                callbacks as *mut _,
            )
        };
    }
}

impl Drop for SoundSource {
    fn drop(&mut self) {
        unsafe { (*PLAYDATE.sound.handle).removeSource.unwrap()(self.handle) };
    }
}
