pub struct Sound {
    #[allow(unused)]
    handle: *const sys::playdate_sound,
    // pub channel: *const playdate_sound_channel,
    // pub fileplayer: *const playdate_sound_fileplayer,
    // pub sample: *const playdate_sound_sample,
    // pub sampleplayer: *const playdate_sound_sampleplayer,
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

impl Sound {
    pub(crate) fn new(handle: *const sys::playdate_sound) -> Self {
        Self { handle }
    }

    // pub getCurrentTime: ::core::option::Option<unsafe extern "C" fn() -> u32>,
    // pub addSource: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         callback: AudioSourceFunction,
    //         context: *mut ::core::ffi::c_void,
    //         stereo: ::core::ffi::c_int,
    //     ) -> *mut SoundSource,
    // >,
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
