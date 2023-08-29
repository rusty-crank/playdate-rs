use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use spin::Mutex;

use crate::{util::Ref, PLAYDATE};

pub(crate) struct PlaydateSoundSource {
    handle: *const sys::playdate_sound_source,
}

impl PlaydateSoundSource {
    pub(crate) fn new(handle: *const sys::playdate_sound_source) -> Self {
        Self { handle }
    }
}

pub struct SoundSource {
    pub(crate) handle: *mut sys::SoundSource,
}

unsafe impl Send for SoundSource {}
unsafe impl Sync for SoundSource {}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct SoundSourcePtr(pub(crate) *const sys::SoundSource);

unsafe impl Send for SoundSourcePtr {}
unsafe impl Sync for SoundSourcePtr {}

type SoundSourcFinishCallbacks = BTreeMap<SoundSourcePtr, Box<dyn FnOnce(&SoundSource) + Send>>;

static SOUND_SOURCE_FINISH_CALLBACKS: Mutex<SoundSourcFinishCallbacks> =
    Mutex::new(BTreeMap::new());

impl SoundSource {
    // pub(crate) fn new(handle: *mut sys::SoundSource) -> Self {
    //     Self { handle }
    // }

    pub(crate) fn new_ref<'a>(handle: *mut sys::SoundSource) -> Ref<'a, Self> {
        Ref::new(Self { handle })
    }

    pub(crate) fn drop_callbacks(&self) {
        SOUND_SOURCE_FINISH_CALLBACKS
            .lock()
            .remove(&SoundSourcePtr(self.handle));
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

    pub fn set_finish_callback(&self, callback: impl Send + FnOnce(&SoundSource) + 'static) {
        let callback = Box::new(callback) as Box<dyn Send + FnOnce(&SoundSource)>;
        SOUND_SOURCE_FINISH_CALLBACKS
            .lock()
            .insert(SoundSourcePtr(self.handle), callback);
        extern "C" fn callback_fn(source: *mut sys::SoundSource) {
            let source = SoundSource::new_ref(source);
            let callback = SOUND_SOURCE_FINISH_CALLBACKS
                .lock()
                .remove(&SoundSourcePtr(source.handle))
                .unwrap();
            callback(&source);
        }
        unsafe {
            (*PLAYDATE.sound.source.handle).setFinishCallback.unwrap()(
                self.handle,
                Some(callback_fn),
            )
        };
    }
}

impl Drop for SoundSource {
    fn drop(&mut self) {
        unsafe { (*PLAYDATE.sound.handle).removeSource.unwrap()(self.handle) };
    }
}
