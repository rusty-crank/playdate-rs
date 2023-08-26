use crate::{util::Ref, PLAYDATE};

use super::{AudioSample, SoundSource};

pub(crate) struct PlaydateSamplePlayer {
    handle: *const sys::playdate_sound_sampleplayer,
}

impl PlaydateSamplePlayer {
    pub(crate) fn new(handle: *const sys::playdate_sound_sampleplayer) -> Self {
        Self { handle }
    }
}

pub struct SamplePlayer {
    handle: *mut sys::SamplePlayer,
}

unsafe impl Send for SamplePlayer {}
unsafe impl Sync for SamplePlayer {}

impl Default for SamplePlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl SamplePlayer {
    /// Allocates a new SamplePlayer.
    pub fn new() -> Self {
        Self {
            handle: unsafe { (*(*PLAYDATE.sound.handle).sampleplayer).newPlayer.unwrap()() },
        }
    }

    fn new_ref<'a>(handle: *mut sys::SamplePlayer) -> Ref<'a, Self> {
        Ref::from(Self { handle })
    }

    pub(crate) fn as_sound_source(&self) -> Ref<SoundSource> {
        SoundSource::new_ref(self.handle as *mut sys::SoundSource)
    }

    /// Returns the length, in seconds, of the sample assigned to player.
    pub fn get_length(&self) -> f32 {
        unsafe { (*PLAYDATE.sound.sample_player.handle).getLength.unwrap()(self.handle) }
    }

    /// Returns false if player is playing a sample, false if not.
    pub fn is_playing(&self) -> bool {
        unsafe { (*PLAYDATE.sound.sample_player.handle).isPlaying.unwrap()(self.handle) != 0 }
    }

    /// Starts playing the sample in player.
    ///
    /// If repeat is greater than one, it loops the given number of times. If zero, it loops endlessly until it is stopped with `SamplePlayer::stop`. If negative one, it does ping-pong looping.
    ///
    /// Sets the playback rate for the sample. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc.
    pub fn play(&self, repeat: usize, rate: f32) {
        unsafe {
            (*PLAYDATE.sound.sample_player.handle).play.unwrap()(self.handle, repeat as _, rate)
        };
    }

    /// Sets a function to be called when playback has completed. This is an alias for `SoundSource::set_finish_callback`.
    pub fn set_finish_callback(&self, callback: impl Send + FnOnce(&Self) + 'static) {
        self.as_sound_source().set_finish_callback(move |x| {
            callback(&Self::new_ref(x.handle as *mut sys::SamplePlayer));
        });
    }

    /// Sets the current offset of the SamplePlayer, in seconds.
    pub fn set_offset(&self, offset: f32) {
        unsafe { (*PLAYDATE.sound.sample_player.handle).setOffset.unwrap()(self.handle, offset) }
    }

    /// Gets the current offset in seconds for player.
    pub fn get_offset(&self) -> f32 {
        unsafe { (*PLAYDATE.sound.sample_player.handle).getOffset.unwrap()(self.handle) }
    }

    // When used with a repeat of -1, does ping-pong looping, with a start and end position in frames.
    pub fn set_playing_range(&self, start: usize, end: usize) {
        unsafe {
            (*PLAYDATE.sound.sample_player.handle).setPlayRange.unwrap()(
                self.handle,
                start as _,
                end as _,
            )
        }
    }

    /// Pauses or resumes playback.
    pub fn set_paused(&self, paused: bool) {
        unsafe {
            (*PLAYDATE.sound.sample_player.handle).setPaused.unwrap()(self.handle, paused as _)
        }
    }

    /// Sets the playback rate for the player. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc. A negative rate produces backwards playback for PCM files, but does not work for ADPCM-encoded files.
    pub fn set_rate(&self, rate: f32) {
        unsafe { (*PLAYDATE.sound.sample_player.handle).setRate.unwrap()(self.handle, rate) }
    }

    /// Gets the playback rate for player.
    pub fn get_rate(&self) -> f32 {
        unsafe { (*PLAYDATE.sound.sample_player.handle).getRate.unwrap()(self.handle) }
    }

    /// Assigns sample to player.
    pub fn set_sample<'a, 'b: 'a>(&'a self, sample: &'b AudioSample) {
        unsafe {
            (*PLAYDATE.sound.sample_player.handle).setSample.unwrap()(self.handle, sample.handle)
        }
    }

    /// Sets the playback volume for left and right channels.
    pub fn set_volume(&self, left: f32, right: f32) {
        unsafe {
            (*PLAYDATE.sound.sample_player.handle).setVolume.unwrap()(self.handle, left, right)
        }
    }

    /// Gets the current left and right channel volume of the sampleplayer.
    pub fn get_volume(&self) -> (f32, f32) {
        let mut left = 0.0;
        let mut right = 0.0;
        unsafe {
            (*PLAYDATE.sound.sample_player.handle).getVolume.unwrap()(
                self.handle,
                &mut left,
                &mut right,
            )
        }
        (left, right)
    }

    /// Stops playing the sample.
    pub fn stop(&self) {
        unsafe { (*PLAYDATE.sound.sample_player.handle).stop.unwrap()(self.handle) }
    }
}

impl Drop for SamplePlayer {
    fn drop(&mut self) {
        self.as_sound_source().drop_callbacks();
        unsafe { (*(*PLAYDATE.sound.handle).sampleplayer).freePlayer.unwrap()(self.handle) }
    }
}
