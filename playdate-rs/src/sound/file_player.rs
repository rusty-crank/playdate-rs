use alloc::{collections::BTreeMap, ffi::CString};
use spin::Mutex;

use crate::{error::Error, util::Ref, PLAYDATE};

use super::{sound_source::SoundSourcePtr, SoundSource};

pub(crate) struct PlaydateFilePlayer {
    handle: *const sys::playdate_sound_fileplayer,
}

impl PlaydateFilePlayer {
    pub(crate) fn new(handle: *const sys::playdate_sound_fileplayer) -> Self {
        Self { handle }
    }
}

pub struct FilePlayer {
    handle: *mut sys::FilePlayer,
}

unsafe impl Send for FilePlayer {}
unsafe impl Sync for FilePlayer {}

impl Default for FilePlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl FilePlayer {
    /// Allocates a new FilePlayer.
    pub fn new() -> Self {
        Self {
            handle: unsafe { (*PLAYDATE.sound.file_player.handle).newPlayer.unwrap()() },
        }
    }

    fn new_ref<'a>(handle: *mut sys::FilePlayer) -> Ref<'a, Self> {
        Ref::new(Self { handle })
    }

    pub(crate) fn as_sound_source(&self) -> Ref<SoundSource> {
        SoundSource::new_ref(self.handle as *mut sys::SoundSource)
    }

    /// Prepares player to stream the file at path.
    pub fn load(&self, path: impl AsRef<str>) -> Result<(), Error> {
        let c_string = CString::new(path.as_ref()).unwrap();
        let result = unsafe {
            (*PLAYDATE.sound.file_player.handle).loadIntoPlayer.unwrap()(
                self.handle,
                c_string.as_ptr(),
            )
        };
        if result != 0 {
            Ok(())
        } else {
            Err(Error::FileNotExists(path.as_ref().to_owned()))
        }
    }

    /// Pause the file player
    pub fn pause(&self) {
        unsafe { (*PLAYDATE.sound.file_player.handle).pause.unwrap()(self.handle) }
    }

    /// Starts playing the file player. If repeat is greater than one, it loops the given number of times. If zero, it loops endlessly until it is stopped with `FilePlayer::stop()`.
    pub fn play(&self, repeat: usize) {
        unsafe { (*PLAYDATE.sound.file_player.handle).play.unwrap()(self.handle, repeat as _) };
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

    /// Sets a function to be called when playback has completed. This is an alias for `SoundSource::set_finish_callback`.
    pub fn set_finish_callback(&self, callback: impl Send + FnOnce(&Self) + 'static) {
        self.as_sound_source().set_finish_callback(move |x| {
            callback(&Self::new_ref(x.handle as *mut sys::FilePlayer));
        });
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
    pub fn fade_volume(
        &self,
        left: f32,
        right: f32,
        len: i32,
        finish_callback: Option<impl Send + FnOnce(&Self) + 'static>,
    ) {
        unsafe extern "C" fn callback_fn(source: *mut sys::SoundSource) {
            let player = FilePlayer::new_ref(source as *mut sys::FilePlayer);
            let callback = FADE_VOLUME_FINISH_CALLBACKS
                .lock()
                .remove(&SoundSourcePtr(source))
                .unwrap();
            callback(&player);
        }
        if let Some(cb) = finish_callback {
            let callback = Box::new(cb) as Box<dyn Send + FnOnce(&Self)>;
            FADE_VOLUME_FINISH_CALLBACKS
                .lock()
                .insert(SoundSourcePtr(self.as_sound_source().handle), callback);

            unsafe {
                (*PLAYDATE.sound.file_player.handle).fadeVolume.unwrap()(
                    self.handle,
                    left,
                    right,
                    len,
                    Some(callback_fn),
                )
            }
        } else {
            unsafe {
                (*PLAYDATE.sound.file_player.handle).fadeVolume.unwrap()(
                    self.handle,
                    left,
                    right,
                    len,
                    None,
                )
            }
        };
    }
}

type FadeVolumeFinishCallbacks = BTreeMap<SoundSourcePtr, Box<dyn FnOnce(&FilePlayer) + Send>>;

static FADE_VOLUME_FINISH_CALLBACKS: Mutex<FadeVolumeFinishCallbacks> = Mutex::new(BTreeMap::new());

impl Drop for FilePlayer {
    fn drop(&mut self) {
        self.as_sound_source().drop_callbacks();
        FADE_VOLUME_FINISH_CALLBACKS
            .lock()
            .remove(&SoundSourcePtr(self.as_sound_source().handle));
        unsafe { (*PLAYDATE.sound.file_player.handle).freePlayer.unwrap()(self.handle) }
    }
}
