use core::cell::RefCell;

use alloc::{collections::BTreeMap, ffi::CString};

use crate::{error::Error, PLAYDATE};

pub struct PlaydateSound {
    #[allow(unused)]
    handle: *const sys::playdate_sound,
    // pub channel: *const playdate_sound_channel,
    file_player: PlaydateFilePlayer,
    // pub sample: *const playdate_sound_sample,
    sample_player: PlaydateSamplePlayer,
    // sampleplayer: *const sys::playdate_sound_sampleplayer,
    // pub synth: *const playdate_sound_synth,
    // pub sequence: *const playdate_sound_sequence,
    // pub effect: *const playdate_sound_effect,
    // pub lfo: *const playdate_sound_lfo,
    // pub envelope: *const playdate_sound_envelope,
    // pub source: *const playdate_sound_source,
    // pub controlsignal: *const playdate_control_signal,
    // pub track: *const playdate_sound_track,
    // pub instrument: *const playdate_sound_instrument,
    // pub signal: *const playdate_sound_signal,
}

impl PlaydateSound {
    pub(crate) fn new(handle: *const sys::playdate_sound) -> Self {
        Self {
            handle,
            file_player: PlaydateFilePlayer::new(unsafe { (*handle).fileplayer }),
            sample_player: PlaydateSamplePlayer::new(unsafe { (*handle).sampleplayer }),
        }
    }

    /// Returns the sound engine’s current time value, in units of sample frames, 44,100 per second.
    pub fn get_current_time(&self) -> u32 {
        unsafe { (*self.handle).getCurrentTime.unwrap()() }
    }

    // pub getCurrentTime: ::core::option::Option<unsafe extern "C" fn() -> u32>,
    // pub addSource: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         callback: AudioSourceFunction,
    //         context: *mut ::core::ffi::c_void,
    //         stereo: ::core::ffi::c_int,
    //     ) -> *mut SoundSource,
    // >,

    // Returns the default channel, where sound sources play if they haven’t been explicity assigned to a different channel.
    // pub(crate) fn get_default_channel(&self) -> Option<sys::SoundChannel> {
    //     let channel = unsafe { (*self.handle).getDefaultChannel.unwrap()() };
    //     if channel.is_null() {
    //         None
    //     } else {
    //         Some(SoundChannel::new(channel))
    //     }
    // }

    // pub getDefaultChannel: ::core::option::Option<unsafe extern "C" fn() -> *mut SoundChannel>,
    // pub addChannel: ::core::option::Option<
    //     unsafe extern "C" fn(channel: *mut SoundChannel) -> ::core::ffi::c_int,
    // >,
    // pub removeChannel: ::core::option::Option<
    //     unsafe extern "C" fn(channel: *mut SoundChannel) -> ::core::ffi::c_int,
    // >,
    // pub setMicCallback: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         callback: RecordCallback,
    //         context: *mut ::core::ffi::c_void,
    //         forceInternal: ::core::ffi::c_int,
    //     ),
    // >,
    // pub getHeadphoneState: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         headphone: *mut ::core::ffi::c_int,
    //         headsetmic: *mut ::core::ffi::c_int,
    //         changeCallback: ::core::option::Option<
    //             unsafe extern "C" fn(headphone: ::core::ffi::c_int, mic: ::core::ffi::c_int),
    //         >,
    //     ),
    // >,
    // pub setOutputsActive: ::core::option::Option<
    //     unsafe extern "C" fn(headphone: ::core::ffi::c_int, speaker: ::core::ffi::c_int),
    // >,
    // pub removeSource: ::core::option::Option<
    //     unsafe extern "C" fn(source: *mut SoundSource) -> ::core::ffi::c_int,
    // >,
}

pub struct PlaydateFilePlayer {
    handle: *const sys::playdate_sound_fileplayer,
}

impl PlaydateFilePlayer {
    fn new(handle: *const sys::playdate_sound_fileplayer) -> Self {
        Self { handle }
    }
}

pub struct FilePlayer {
    handle: *mut sys::FilePlayer,
}

unsafe impl Send for FilePlayer {}
unsafe impl Sync for FilePlayer {}

impl FilePlayer {
    /// Allocates a new FilePlayer.
    pub fn new() -> Self {
        Self {
            handle: unsafe { (*PLAYDATE.sound.file_player.handle).newPlayer.unwrap()() },
        }
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

    // void playdate->sound->fileplayer->setFinishCallback(FilePlayer* player, sndCallbackProc callback);

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

    // void playdate->sound->fileplayer->fadeVolume(FilePlayer* player, float left, float right, int32_t len, sndCallbackProc finishCallback);
}

impl Drop for FilePlayer {
    fn drop(&mut self) {
        unsafe { (*PLAYDATE.sound.file_player.handle).freePlayer.unwrap()(self.handle) }
    }
}

pub struct PlaydateSamplePlayer {
    handle: *const sys::playdate_sound_sampleplayer,
}

impl PlaydateSamplePlayer {
    fn new(handle: *const sys::playdate_sound_sampleplayer) -> Self {
        Self { handle }
    }
}

pub struct SamplePlayer {
    handle: *mut sys::SamplePlayer,
}

unsafe impl Send for SamplePlayer {}
unsafe impl Sync for SamplePlayer {}

impl SamplePlayer {
    /// Allocates a new SamplePlayer.
    pub fn new() -> Self {
        Self {
            handle: unsafe { (*(*PLAYDATE.sound.handle).sampleplayer).newPlayer.unwrap()() },
        }
    }
}

impl Drop for SamplePlayer {
    fn drop(&mut self) {
        unsafe { (*(*PLAYDATE.sound.handle).sampleplayer).freePlayer.unwrap()(self.handle) }
    }
}
