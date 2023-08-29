use euclid::default::{Point2D, Vector2D};

pub struct PlaydateDisplay {
    handle: *const sys::playdate_display,
}

impl PlaydateDisplay {
    pub(crate) fn new(handle: *const sys::playdate_display) -> Self {
        Self { handle }
    }

    /// Returns the height of the display, taking the current scale into account; e.g., if the scale is 2, this function returns 120 instead of 240.
    pub fn get_width(&self) -> u32 {
        unsafe { (*self.handle).getWidth.unwrap()() as _ }
    }

    /// Returns the width of the display, taking the current scale into account; e.g., if the scale is 2, this function returns 200 instead of 400.
    pub fn get_height(&self) -> u32 {
        unsafe { (*self.handle).getHeight.unwrap()() as _ }
    }

    /// Sets the nominal refresh rate in frames per second. Default is 20 fps, the maximum rate supported by the hardware for full-frame updates.
    pub fn set_refresh_rate(&self, rate: f32) {
        unsafe { (*self.handle).setRefreshRate.unwrap()(rate) }
    }

    /// If flag evaluates to true, the frame buffer is drawn inverted—black instead of white, and vice versa.
    pub fn set_inverted(&self, flag: bool) {
        unsafe { (*self.handle).setInverted.unwrap()(flag as _) }
    }

    /// Sets the display scale factor. Valid values for scale are 1, 2, 4, and 8.
    ///
    /// The top-left corner of the frame buffer is scaled up to fill the display; e.g., if the scale is set to 4, the pixels in rectangle \[0,100\] x \[0,60\] are drawn on the screen as 4 x 4 squares.
    pub fn set_scale(&self, s: u32) {
        unsafe { (*self.handle).setScale.unwrap()(s) }
    }

    /// Adds a mosaic effect to the display. Valid x and y values are between 0 and 3, inclusive.
    pub fn set_mosaic(&self, effect: Vector2D<u32>) {
        debug_assert!(
            effect.x < 4 && effect.y < 4,
            "invalid mosaic effect: {:?}",
            effect
        );
        unsafe { (*self.handle).setMosaic.unwrap()(effect.x, effect.y) }
    }

    /// Flips the display on the x or y axis, or both.
    pub fn set_flipped(&self, x: bool, y: bool) {
        unsafe { (*self.handle).setFlipped.unwrap()(x as _, y as _) }
    }

    /// Offsets the display by the given amount. Areas outside of the displayed area are filled with the current background color.
    pub fn set_offset(&self, delta: Point2D<i32>) {
        unsafe { (*self.handle).setOffset.unwrap()(delta.x, delta.y) }
    }
}

pub const DISPLAY_WIDTH: u32 = 400;
pub const DISPLAY_HEIGHT: u32 = 240;
