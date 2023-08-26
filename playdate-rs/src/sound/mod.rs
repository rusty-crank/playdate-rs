mod file_player;
mod sample;
mod sample_player;
mod sound_source;

pub use file_player::FilePlayer;
pub use sample::{AudioSample, AudioSampleData, SoundFormat};
pub use sample_player::SamplePlayer;
pub use sound_source::SoundSource;

pub struct PlaydateSound {
    #[allow(unused)]
    handle: *const sys::playdate_sound,
    // pub channel: *const playdate_sound_channel,
    file_player: file_player::PlaydateFilePlayer,
    sample: sample::PlaydateSample,
    sample_player: sample_player::PlaydateSamplePlayer,
    // pub synth: *const playdate_sound_synth,
    // pub sequence: *const playdate_sound_sequence,
    // pub effect: *const playdate_sound_effect,
    // pub lfo: *const playdate_sound_lfo,
    // pub envelope: *const playdate_sound_envelope,
    source: sound_source::PlaydateSoundSource,
    // pub controlsignal: *const playdate_control_signal,
    // pub track: *const playdate_sound_track,
    // pub instrument: *const playdate_sound_instrument,
    // pub signal: *const playdate_sound_signal,
}

impl PlaydateSound {
    pub(crate) fn new(handle: *const sys::playdate_sound) -> Self {
        Self {
            handle,
            file_player: file_player::PlaydateFilePlayer::new(unsafe { (*handle).fileplayer }),
            sample: sample::PlaydateSample::new(unsafe { (*handle).sample }),
            sample_player: sample_player::PlaydateSamplePlayer::new(unsafe {
                (*handle).sampleplayer
            }),
            source: sound_source::PlaydateSoundSource::new(unsafe { (*handle).source }),
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
