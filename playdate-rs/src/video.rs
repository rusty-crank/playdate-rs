use alloc::{borrow::ToOwned, ffi::CString, string::String};

use crate::{graphics::Bitmap, util::Ref, PLAYDATE};

pub struct Video {
    handle: *const sys::playdate_video,
}

impl Video {
    pub(crate) fn new(handle: *const sys::playdate_video) -> Self {
        Self { handle }
    }

    /// Opens the pdv file at path and returns a new video player object for rendering its frames.
    pub fn load(&self, path: impl AsRef<str>) -> Result<VideoPlayer, String> {
        let c_string = CString::new(path.as_ref()).unwrap();
        let player = unsafe { (*self.handle).loadVideo.unwrap()(c_string.as_ptr()) };
        if player.is_null() {
            Err("Failed to load video".to_owned())
        } else {
            Ok(VideoPlayer::new(player))
        }
    }

    /// Frees the given video player.
    pub(crate) fn free_player(&self, player: *mut sys::LCDVideoPlayer) {
        unsafe { (*self.handle).freePlayer.unwrap()(player) }
    }

    /// Sets the rendering destination for the video player to the given bitmap. If the function fails, it returns 0 and sets an error message that can be read via getError().
    pub(crate) fn set_context(
        &self,
        player: *mut sys::LCDVideoPlayer,
        context: *mut sys::LCDBitmap,
    ) -> i32 {
        unsafe { (*self.handle).setContext.unwrap()(player, context) }
    }

    /// Sets the rendering destination for the video player to the screen.
    pub fn use_screen_context(&self, player: &VideoPlayer) {
        unsafe { (*self.handle).useScreenContext.unwrap()(player.handle) }
    }

    /// Renders frame number n into the current context.
    pub(crate) fn render_frame(&self, player: &VideoPlayer, n: i32) -> Result<(), String> {
        let result = unsafe { (*self.handle).renderFrame.unwrap()(player.handle, n) };
        if result != 0 {
            Ok(())
        } else {
            let err = self.get_error(player.handle).unwrap();
            Err(err.to_owned())
        }
    }

    /// Returns text describing the most recent error.
    pub(crate) fn get_error(&self, player: *mut sys::LCDVideoPlayer) -> Option<&str> {
        let c_string = unsafe { (*self.handle).getError.unwrap()(player) };
        if c_string.is_null() {
            None
        } else {
            let c_str = unsafe { ::core::ffi::CStr::from_ptr(c_string) };
            Some(c_str.to_str().unwrap())
        }
    }

    /// Retrieves information about the video, by passing in (possibly NULL) value pointers.
    pub(crate) fn get_info(&self, player: *mut sys::LCDVideoPlayer) -> VideoPlayerInfo {
        let mut info = VideoPlayerInfo::default();
        unsafe {
            (*self.handle).getInfo.unwrap()(
                player,
                &mut info.width,
                &mut info.height,
                &mut info.frame_rate,
                &mut info.frame_count,
                &mut info.current_frame,
            )
        };
        info
    }

    /// Gets the rendering destination for the video player. If no rendering context has been setallocates a context bitmap with the same dimensions as the vieo will be allocated.
    pub(crate) fn get_context(&self, player: *mut sys::LCDVideoPlayer) -> *mut sys::LCDBitmap {
        unsafe { (*self.handle).getContext.unwrap()(player) }
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

impl VideoPlayer {
    fn new(handle: *mut sys::LCDVideoPlayer) -> Self {
        Self { handle }
    }

    /// Sets the rendering destination for the video player to the given bitmap.
    pub fn set_context<'a, 'b: 'a>(&'a self, context: &'b Bitmap) -> Result<(), String> {
        let result = PLAYDATE
            .graphics
            .video
            .set_context(self.handle, context.as_ref().handle);
        if result != 0 {
            Ok(())
        } else {
            Err(self.get_error().unwrap())
        }
    }

    /// Renders frame number n into the current context.
    pub fn render_frame(&self, n: i32) -> Result<(), String> {
        PLAYDATE.graphics.video.render_frame(self, n)
    }

    /// Returns text describing the most recent error.
    pub fn get_error(&self) -> Option<String> {
        PLAYDATE
            .graphics
            .video
            .get_error(self.handle)
            .map(|s| s.to_owned())
    }

    /// Retrieves information about the video, by passing in (possibly NULL) value pointers.
    pub fn get_info(&self) -> VideoPlayerInfo {
        PLAYDATE.graphics.video.get_info(self.handle)
    }

    /// Gets the rendering destination for the video player. If no rendering context has been setallocates a context bitmap with the same dimensions as the vieo will be allocated.
    pub fn get_context(&self) -> Ref<Bitmap> {
        let ptr = PLAYDATE.graphics.video.get_context(self.handle);
        // FIXME: ptr maybe malloced
        Bitmap::from_ref(ptr)
    }
}

impl Drop for VideoPlayer {
    fn drop(&mut self) {
        PLAYDATE.graphics.video.free_player(self.handle);
    }
}
