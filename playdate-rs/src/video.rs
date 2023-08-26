use crate::error::Error;

use alloc::{borrow::ToOwned, ffi::CString, string::String};

use crate::{graphics::Bitmap, util::Ref, PLAYDATE};

pub struct PlaydateVideo {
    handle: *const sys::playdate_video,
}

impl PlaydateVideo {
    pub(crate) fn new(handle: *const sys::playdate_video) -> Self {
        Self { handle }
    }

    /// Sets the rendering destination for the video player to the screen.
    pub fn use_screen_context(&self, player: &VideoPlayer) {
        unsafe { (*self.handle).useScreenContext.unwrap()(player.handle) }
    }
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct VideoPlayerInfo {
    pub width: i32,
    pub height: i32,
    pub frame_rate: f32,
    pub frame_count: i32,
    pub current_frame: i32,
}

#[derive(PartialEq, Eq, Debug)]
pub struct VideoPlayer {
    handle: *mut sys::LCDVideoPlayer,
}

unsafe impl Send for VideoPlayer {}
unsafe impl Sync for VideoPlayer {}

impl VideoPlayer {
    /// Opens the pdv file at path and returns a new video player object for rendering its frames.
    pub fn load(&self, path: impl AsRef<str>) -> Result<Self, Error> {
        let c_string = CString::new(path.as_ref()).unwrap();
        let handle =
            unsafe { (*PLAYDATE.graphics.video.handle).loadVideo.unwrap()(c_string.as_ptr()) };
        if handle.is_null() {
            Err(Error::FileNotExists("Failed to load video".to_owned()))
        } else {
            Ok(Self { handle })
        }
    }

    /// Sets the rendering destination for the video player to the given bitmap.
    pub fn set_context<'a, 'b: 'a>(&'a self, context: &'b Bitmap) -> Result<(), Error> {
        let result = unsafe {
            (*PLAYDATE.graphics.video.handle).setContext.unwrap()(self.handle, context.handle)
        };
        if result != 0 {
            Ok(())
        } else {
            Err(Error::Unknown(self.get_error().unwrap()))
        }
    }

    /// Gets the rendering destination for the video player. If no rendering context has been setallocates a context bitmap with the same dimensions as the vieo will be allocated.
    pub fn get_context(&self) -> Ref<Bitmap> {
        let ptr = unsafe { (*PLAYDATE.graphics.video.handle).getContext.unwrap()(self.handle) };
        // FIXME: ptr maybe malloced
        Bitmap::from_ref(ptr)
    }

    /// Renders frame number n into the current context.
    pub fn render_frame(&self, n: usize) -> Result<(), Error> {
        let result =
            unsafe { (*PLAYDATE.graphics.video.handle).renderFrame.unwrap()(self.handle, n as _) };
        if result != 0 {
            Ok(())
        } else {
            let err = self.get_error().unwrap();
            Err(Error::Unknown(err.to_owned()))
        }
    }

    /// Returns text describing the most recent error.
    pub fn get_error(&self) -> Option<String> {
        let c_string = unsafe { (*PLAYDATE.graphics.video.handle).getError.unwrap()(self.handle) };
        if c_string.is_null() {
            None
        } else {
            let c_str = unsafe { ::core::ffi::CStr::from_ptr(c_string) };
            Some(c_str.to_str().unwrap().to_owned())
        }
    }

    /// Retrieves information about the video, by passing in (possibly NULL) value pointers.
    pub fn get_info(&self) -> VideoPlayerInfo {
        let mut info = VideoPlayerInfo::default();
        unsafe {
            (*PLAYDATE.graphics.video.handle).getInfo.unwrap()(
                self.handle,
                &mut info.width,
                &mut info.height,
                &mut info.frame_rate,
                &mut info.frame_count,
                &mut info.current_frame,
            )
        };
        info
    }
}

impl Drop for VideoPlayer {
    fn drop(&mut self) {
        unsafe { (*PLAYDATE.graphics.video.handle).freePlayer.unwrap()(self.handle) }
    }
}
