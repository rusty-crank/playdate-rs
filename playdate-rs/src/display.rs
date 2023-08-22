pub struct Display {
    handle: *const sys::playdate_display,
}

impl Display {
    pub(crate) fn new(handle: *const sys::playdate_display) -> Self {
        Self { handle }
    }

    /// Returns the height of the display, taking the current scale into account; e.g., if the scale is 2, this function returns 120 instead of 240.
    pub fn get_width(&self) -> i32 {
        unsafe { (*self.handle).getWidth.unwrap()() }
    }

    /// Returns the width of the display, taking the current scale into account; e.g., if the scale is 2, this function returns 200 instead of 400.
    pub fn get_height(&self) -> i32 {
        unsafe { (*self.handle).getHeight.unwrap()() }
    }

    /// Sets the nominal refresh rate in frames per second. Default is 20 fps, the maximum rate supported by the hardware for full-frame updates.
    pub fn set_refresh_rate(&self, rate: f32) {
        unsafe { (*self.handle).setRefreshRate.unwrap()(rate) }
    }

    /// If flag evaluates to true, the frame buffer is drawn inverted—black instead of white, and vice versa.
    pub fn set_inverted(&self, flag: i32) {
        unsafe { (*self.handle).setInverted.unwrap()(flag) }
    }

    /// Sets the display scale factor. Valid values for scale are 1, 2, 4, and 8.
    ///
    /// The top-left corner of the frame buffer is scaled up to fill the display; e.g., if the scale is set to 4, the pixels in rectangle [0,100] x [0,60] are drawn on the screen as 4 x 4 squares.
    pub fn set_scale(&self, s: u32) {
        unsafe { (*self.handle).setScale.unwrap()(s) }
    }

    /// Adds a mosaic effect to the display. Valid x and y values are between 0 and 3, inclusive.
    pub fn set_mosaic(&self, x: u32, y: u32) {
        unsafe { (*self.handle).setMosaic.unwrap()(x, y) }
    }

    /// Flips the display on the x or y axis, or both.
    pub fn set_flipped(&self, x: i32, y: i32) {
        unsafe { (*self.handle).setFlipped.unwrap()(x, y) }
    }

    /// Offsets the display by the given amount. Areas outside of the displayed area are filled with the current background color.
    pub fn set_offset(&self, x: i32, y: i32) {
        unsafe { (*self.handle).setOffset.unwrap()(x, y) }
    }
}

pub const DISPLAY_WIDTH: i32 = 400;
pub const DISPLAY_HEIGHT: i32 = 240;
