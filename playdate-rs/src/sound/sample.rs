use crate::fs::AsPath;
use crate::{error::Error, PLAYDATE};

use alloc::ffi::CString;
pub use sys::SoundFormat;

pub struct PlaydateSample {
    handle: *const sys::playdate_sound_sample,
}

impl PlaydateSample {
    pub(crate) fn new(handle: *const sys::playdate_sound_sample) -> Self {
        Self { handle }
    }
}

pub struct AudioSample<'a> {
    pub(crate) handle: *mut sys::AudioSample,
    #[allow(dead_code)]
    buf: Option<&'a [u8]>,
}

impl<'a> AudioSample<'a> {
    pub fn from_data(buf: &'a [u8], format: SoundFormat, sample_rate: u32) -> Self {
        let length = buf.len();
        let handle = unsafe {
            (*PLAYDATE.sound.sample.handle).newSampleFromData.unwrap()(
                buf.as_ptr() as _,
                format,
                sample_rate,
                length as _,
                0,
            )
        };
        Self {
            handle,
            buf: Some(buf),
        }
    }

    /// Allocates and returns a new AudioSample with a buffer large enough to load a file of length bytes.
    pub fn new(length: usize) -> Self {
        Self {
            handle: unsafe {
                (*PLAYDATE.sound.sample.handle).newSampleBuffer.unwrap()(length as _)
            },
            buf: None,
        }
    }

    /// Allocates and returns a new AudioSample, with the sound data loaded in memory. If there is no file at path, the function returns null.
    pub fn load(path: impl AsPath) -> Result<Self, Error> {
        let c_string = CString::new(path.as_str().as_ref()).unwrap();
        let handle = unsafe { (*PLAYDATE.sound.sample.handle).load.unwrap()(c_string.as_ptr()) };
        if handle.is_null() {
            Err(Error::FileNotExists(path.as_str().into_owned()))
        } else {
            Ok(Self { handle, buf: None })
        }
    }

    // AudioSample* playdate->sound->sample->newSampleFromData(uint8_t* data, SoundFormat format, uint32_t sampleRate, int byteCount)
    // Returns a new AudioSample referencing the given audio data. The sample keeps a pointer to the data instead of copying it, so the data must remain valid while the sample is active. format is one of the following values:

    /// Loads the sound data from the file at path into an existing AudioSample, sample.
    pub fn load_from_file(&mut self, path: impl AsPath) -> Result<(), Error> {
        let c_string = CString::new(path.as_str().as_ref()).unwrap();
        let result = unsafe {
            (*PLAYDATE.sound.sample.handle).loadIntoSample.unwrap()(self.handle, c_string.as_ptr())
        };
        if result != 0 {
            Ok(())
        } else {
            Err(Error::FileNotExists(path.as_str().into_owned()))
        }
    }

    /// Retrieves the sample’s data, format, sample rate, and data length.
    pub fn get_data(&self) -> AudioSampleData {
        let mut data = core::ptr::null_mut();
        let mut format = sys::SoundFormat(0);
        let mut sample_rate = 0;
        let mut byte_length = 0;
        unsafe {
            (*PLAYDATE.sound.sample.handle).getData.unwrap()(
                self.handle,
                &mut data,
                &mut format,
                &mut sample_rate,
                &mut byte_length,
            )
        };
        AudioSampleData {
            data: unsafe { core::slice::from_raw_parts_mut(data, byte_length as _) },
            format,
            sample_rate,
        }
    }

    /// Returns the length, in seconds, of sample.
    pub fn get_length(&self) -> f32 {
        unsafe { (*PLAYDATE.sound.sample.handle).getLength.unwrap()(self.handle) }
    }
}

impl<'a> Drop for AudioSample<'a> {
    fn drop(&mut self) {
        unsafe { (*PLAYDATE.sound.sample.handle).freeSample.unwrap()(self.handle) }
    }
}

#[derive(Debug)]
pub struct AudioSampleData<'a> {
    pub data: &'a mut [u8],
    pub format: sys::SoundFormat,
    pub sample_rate: u32,
}
