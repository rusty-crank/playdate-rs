use alloc::boxed::Box;
use alloc::ffi::CString;
use alloc::sync::Arc;
use core::cell::RefCell;
use core::ffi::c_void;

use crate::fs::AsPath;
use crate::util::callback_to_async::CallbackFuture;
use crate::{error::Error, PLAYDATE};

use super::SoundSource;

pub(crate) struct PlaydateFilePlayer {
    handle: *const sys::playdate_sound_fileplayer,
}

impl PlaydateFilePlayer {
    pub(crate) fn new(handle: *const sys::playdate_sound_fileplayer) -> Self {
        Self { handle }
    }
}

struct Callbacks {
    fade: Option<Box<dyn FnMut()>>,
}

pub struct FilePlayer {
    handle: *mut sys::FilePlayer,
    source: SoundSource,
    callbacks: Arc<RefCell<Callbacks>>,
}

unsafe impl Send for FilePlayer {}
unsafe impl Sync for FilePlayer {}

impl Default for FilePlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl FilePlayer {
    /// Create a new FilePlayer.
    pub fn new() -> Self {
        let handle = unsafe { (*PLAYDATE.sound.file_player.handle).newPlayer.unwrap()() };
        Self {
            handle,
            source: SoundSource::new(handle as _),
            callbacks: Arc::new(RefCell::new(Callbacks { fade: None })),
        }
    }

    pub fn sound_source(&self) -> &SoundSource {
        &self.source
    }

    /// Prepares player to stream the file at path.
    pub fn load_from_file(&self, path: impl AsPath) -> Result<(), Error> {
        let c_string = CString::new(path.as_str().as_ref()).unwrap();
        let result = unsafe {
            (*PLAYDATE.sound.file_player.handle).loadIntoPlayer.unwrap()(
                self.handle,
                c_string.as_ptr(),
            )
        };
        if result != 0 {
            Ok(())
        } else {
            Err(Error::FileNotExists(path.as_str().into_owned()))
        }
    }

    /// Create a new FilePlayer and load an audio file.
    pub fn load(path: impl AsPath) -> Result<Self, Error> {
        let player = Self::new();
        player.load_from_file(path)?;
        Ok(player)
    }

    /// Pause the file player
    pub fn pause(&self) {
        unsafe { (*PLAYDATE.sound.file_player.handle).pause.unwrap()(self.handle) }
    }

    /// Starts playing the file player. If repeat is greater than one, it loops the given number of times. If zero, it loops endlessly until it is stopped with `FilePlayer::stop()`.
    pub async fn play(&self, repeat: usize) {
        let future = CallbackFuture::<()>::new();
        let handle = future.get_handle();
        self.source.set_finish_callback(move || {
            CallbackFuture::<()>::resolve(handle, ());
        });
        unsafe { (*PLAYDATE.sound.file_player.handle).play.unwrap()(self.handle, repeat as _) };
        future.await;
    }

    /// Returns true if player is playing, false if not.
    pub fn is_playing(&self) -> bool {
        unsafe { (*PLAYDATE.sound.file_player.handle).isPlaying.unwrap()(self.handle) != 0 }
    }

    /// Sets the buffer length of player to `buffer_len` seconds;
    pub fn set_buffer_length(&self, buffer_len: f32) {
        unsafe {
            (*PLAYDATE.sound.file_player.handle)
                .setBufferLength
                .unwrap()(self.handle, buffer_len)
        }
    }

    /// Returns the length, in seconds, of the file loaded into player.
    pub fn get_length(&self) -> f32 {
        unsafe { (*PLAYDATE.sound.file_player.handle).getLength.unwrap()(self.handle) }
    }

    /// Returns true if player has underrun, false if not.
    pub fn did_underrun(&self) -> bool {
        unsafe { (*PLAYDATE.sound.file_player.handle).didUnderrun.unwrap()(self.handle) != 0 }
    }

    /// Sets the start and end of the loop region for playback, in seconds. If end is omitted, the end of the file is used.
    pub fn set_loop_range(&self, start: f32, end: Option<f32>) {
        unsafe {
            (*PLAYDATE.sound.file_player.handle).setLoopRange.unwrap()(
                self.handle,
                start,
                end.unwrap_or(0.0),
            )
        }
    }

    /// Sets the current offset in seconds.
    pub fn set_offset(&self, offset: f32) {
        unsafe { (*PLAYDATE.sound.file_player.handle).setOffset.unwrap()(self.handle, offset) }
    }

    /// Gets the current offset in seconds for player.
    pub fn get_offset(&self) -> f32 {
        unsafe { (*PLAYDATE.sound.file_player.handle).getOffset.unwrap()(self.handle) }
    }

    /// Sets the playback rate for the player. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc. Unlike sampleplayers, fileplayers can’t play in reverse (i.e., rate < 0).
    pub fn set_rate(&self, rate: f32) {
        unsafe { (*PLAYDATE.sound.file_player.handle).setRate.unwrap()(self.handle, rate) }
    }

    /// Gets the playback rate for player.
    pub fn get_rate(&self) -> f32 {
        unsafe { (*PLAYDATE.sound.file_player.handle).getRate.unwrap()(self.handle) }
    }

    /// If flag evaluates to true, the player will restart playback (after an audible stutter) as soon as data is available.
    pub fn set_stop_on_underrun(&self, flag: bool) {
        unsafe {
            (*PLAYDATE.sound.file_player.handle)
                .setStopOnUnderrun
                .unwrap()(self.handle, flag as _)
        }
    }

    /// Sets the playback volume for left and right channels of player.
    pub fn set_volume(&self, left: f32, right: f32) {
        unsafe { (*PLAYDATE.sound.file_player.handle).setVolume.unwrap()(self.handle, left, right) }
    }

    /// Gets the left and right channel playback volume for player.
    pub fn get_volume(&self) -> (f32, f32) {
        let mut left = 0.0;
        let mut right = 0.0;
        unsafe {
            (*PLAYDATE.sound.file_player.handle).getVolume.unwrap()(
                self.handle,
                &mut left,
                &mut right,
            )
        }
        (left, right)
    }

    /// Stops playing the file.
    pub fn stop(&self) {
        unsafe { (*PLAYDATE.sound.file_player.handle).stop.unwrap()(self.handle) }
    }

    /// Changes the volume of the fileplayer to left and right over a length of len sample frames, then calls the provided callback (if set).
    pub async fn fade_volume(&self, left: f32, right: f32, len: i32) {
        let future = CallbackFuture::<()>::new();
        let handle = future.get_handle();
        let callback: Box<dyn FnMut()> = Box::new(move || {
            CallbackFuture::<()>::resolve(handle, ());
        });
        self.callbacks.borrow_mut().fade = Some(callback);
        let callbacks = self.callbacks.as_ptr() as *const RefCell<Callbacks>;
        unsafe extern "C" fn callback_fn(_source: *mut sys::SoundSource, userdata: *mut c_void) {
            let callback = unsafe { &*(userdata as *const RefCell<Callbacks>) };
            if let Some(ref mut f) = callback.borrow_mut().fade {
                f();
            }
        }
        unsafe {
            (*PLAYDATE.sound.file_player.handle).fadeVolume.unwrap()(
                self.handle,
                left,
                right,
                len,
                Some(callback_fn),
                callbacks as *mut _,
            )
        }
        future.await;
    }
}

impl Drop for FilePlayer {
    fn drop(&mut self) {
        unsafe { (*PLAYDATE.sound.file_player.handle).freePlayer.unwrap()(self.handle) }
    }
}
