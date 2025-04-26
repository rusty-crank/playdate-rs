use core::ffi::{c_char, c_void};

use crate::math::{Rect, Vec2};
use alloc::ffi::CString;

use crate::util::Ref;

pub use sys::{
    LCDBitmapDrawMode as BitmapDrawMode, LCDBitmapFlip as BitmapFlip, LCDColor as ColorOrPattern,
    LCDLineCapStyle as LineCapStyle, LCDPattern as Pattern, LCDPolygonFillRule as PolygonFillRule,
    LCDSolidColor as Color, PDTextAlignment as TextAlign, PDTextWrappingMode as TextWrap,
    LCD_COLUMNS, LCD_ROWS, LCD_ROWSIZE,
};

use crate::error::Error;

mod bitmap;
mod font;
mod tilemap;
mod video;

pub use bitmap::{Bitmap, BitmapData, BitmapTable, BitmapTableInfo};
pub use font::{Font, FontGlyph, FontPage};
pub use tilemap::TileMap;
pub use video::{VideoPlayer, VideoStreamPlayer};

pub struct PlaydateGraphics {
    handle: *const sys::playdate_graphics,
    pub video: video::PlaydateVideo,
}

impl PlaydateGraphics {
    pub(crate) fn new(handle: *const sys::playdate_graphics) -> Self {
        Self {
            handle,
            video: video::PlaydateVideo::new(unsafe { (*handle).video }),
        }
    }

    /// Clears the entire display, filling it with color.
    pub fn clear(&self, color: impl Into<ColorOrPattern>) {
        unsafe {
            ((*self.handle).clear.unwrap())(color.into());
        }
    }

    /// Sets the background color shown when the display is offset or for clearing dirty areas in the sprite system.
    pub fn set_background_color(&self, color: Color) {
        unsafe {
            ((*self.handle).setBackgroundColor.unwrap())(color);
        }
    }

    /// Sets the stencil used for drawing. For a tiled stencil, use setStencilImage() instead.
    pub fn set_stencil(&self, stencil: impl AsRef<Bitmap>) {
        unsafe {
            ((*self.handle).setStencil.unwrap())(stencil.as_ref().handle);
        }
    }

    /// Sets the mode used for drawing bitmaps. Note that text drawing uses bitmaps, so this affects how fonts are displayed as well.
    pub fn set_draw_mode(&self, mode: BitmapDrawMode) -> BitmapDrawMode {
        unsafe { ((*self.handle).setDrawMode.unwrap())(mode) }
    }

    /// Offsets the origin point for all drawing calls to x, y (can be negative).
    /// This is useful, for example, for centering a "camera" on a sprite that is moving around a world larger than the screen.
    pub fn set_draw_offset(&self, delta: Vec2<i32>) {
        unsafe {
            ((*self.handle).setDrawOffset.unwrap())(delta.x, delta.y);
        }
    }

    /// Sets the current clip rect, using world coordinates—​that is, the given rectangle will be translated by the current drawing offset. The clip rect is cleared at the beginning of each update.
    pub fn set_clip_rect(&self, rect: Rect<i32>) {
        unsafe {
            ((*self.handle).setClipRect.unwrap())(rect.x, rect.y, rect.width, rect.height);
        }
    }

    /// Clears the current clip rect.
    pub fn clear_clip_rect(&self) {
        unsafe {
            ((*self.handle).clearClipRect.unwrap())();
        }
    }

    /// Sets the end cap style used in the line drawing functions.
    pub fn set_line_cap_style(&self, end_cap_style: LineCapStyle) {
        unsafe {
            ((*self.handle).setLineCapStyle.unwrap())(end_cap_style);
        }
    }

    /// Sets the font to use in subsequent drawText calls.
    pub fn set_font(&self, font: &Font) {
        unsafe {
            ((*self.handle).setFont.unwrap())(font.handle);
        }
    }

    /// Sets the tracking to use when drawing text.
    pub fn set_text_tracking(&self, tracking: i32) {
        unsafe {
            ((*self.handle).setTextTracking.unwrap())(tracking);
        }
    }

    /// Gets the tracking used when drawing text.
    pub fn get_text_tracking(&self) -> i32 {
        unsafe { ((*self.handle).getTextTracking.unwrap())() }
    }

    /// Push a new drawing context for drawing into the given bitmap. If target is nil, the drawing functions will use the display framebuffer.
    pub fn push_context(&self, target: impl AsRef<Bitmap>) {
        unsafe {
            ((*self.handle).pushContext.unwrap())(target.as_ref().handle);
        }
    }

    /// Pops a context off the stack (if any are left), restoring the drawing settings from before the context was pushed.
    pub fn pop_context(&self) {
        unsafe {
            ((*self.handle).popContext.unwrap())();
        }
    }

    /// Draws the bitmap with its upper-left corner at location x, y, using the given flip orientation.
    pub fn draw_bitmap(&self, bitmap: impl AsRef<Bitmap>, pos: Vec2<i32>, flip: BitmapFlip) {
        unsafe {
            ((*self.handle).drawBitmap.unwrap())(bitmap.as_ref().handle, pos.x, pos.y, flip);
        }
    }

    /// Draws the bitmap with its upper-left corner at location x, y tiled inside a width by height rectangle.
    pub fn tile_bitmap(&self, bitmap: impl AsRef<Bitmap>, rect: Rect<i32>, flip: BitmapFlip) {
        unsafe {
            ((*self.handle).tileBitmap.unwrap())(
                bitmap.as_ref().handle,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                flip,
            );
        }
    }

    /// Draws a line from x1, y1 to x2, y2 with a stroke width of width.
    pub fn draw_line(
        &self,
        start: Vec2<i32>,
        end: Vec2<i32>,
        width: i32,
        color: impl Into<ColorOrPattern>,
    ) {
        unsafe {
            ((*self.handle).drawLine.unwrap())(start.x, start.y, end.x, end.y, width, color.into());
        }
    }

    /// Draws a filled triangle with points at x1, y1, x2, y2, and x3, y3.
    #[allow(clippy::too_many_arguments)]
    pub fn fill_triangle(
        &self,
        pos1: Vec2<i32>,
        pos2: Vec2<i32>,
        pos3: Vec2<i32>,
        color: impl Into<ColorOrPattern>,
    ) {
        unsafe {
            ((*self.handle).fillTriangle.unwrap())(
                pos1.x,
                pos1.y,
                pos2.x,
                pos2.y,
                pos3.x,
                pos3.y,
                color.into(),
            );
        }
    }

    /// Draws a pixel at x, y.
    pub fn draw_pixel(&self, pos: Vec2<i32>, color: Color) {
        let fb = self.get_frame();
        let byte_ptr = unsafe { fb.add((pos.y * LCD_ROWSIZE as i32 + (pos.x >> 3)) as usize) };
        if color == Color::Black {
            unsafe { *byte_ptr &= !(1 << (7 - (pos.x & 7))) };
        } else {
            unsafe { *byte_ptr |= 1 << (7 - (pos.x & 7)) };
        }
    }

    /// Draws a width by height rect at x, y.
    pub fn draw_rect(&self, rect: Rect<i32>, color: impl Into<ColorOrPattern>) {
        unsafe {
            ((*self.handle).drawRect.unwrap())(
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                color.into(),
            );
        }
    }

    /// Draws a filled width by height rect at x, y.
    pub fn fill_rect(&self, rect: Rect<i32>, color: impl Into<ColorOrPattern>) {
        unsafe {
            ((*self.handle).fillRect.unwrap())(
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                color.into(),
            );
        }
    }

    /// Draws an ellipse inside the rectangle {x, y, width, height} of width lineWidth (inset from the rectangle bounds). If startAngle != _endAngle, this draws an arc between the given angles. Angles are given in degrees, clockwise from due north.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_ellipse(
        &self,
        rect: Rect<i32>,
        line_width: i32,
        start_angle: f32,
        end_angle: f32,
        color: impl Into<ColorOrPattern>,
    ) {
        unsafe {
            ((*self.handle).drawEllipse.unwrap())(
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                line_width,
                start_angle,
                end_angle,
                color.into(),
            );
        }
    }

    /// Fills an ellipse inside the rectangle {x, y, width, height}. If startAngle != _endAngle, this draws a wedge/Pacman between the given angles. Angles are given in degrees, clockwise from due north.
    #[allow(clippy::too_many_arguments)]
    pub fn fill_ellipse(
        &self,
        rect: Rect<i32>,
        start_angle: f32,
        end_angle: f32,
        color: impl Into<ColorOrPattern>,
    ) {
        unsafe {
            ((*self.handle).fillEllipse.unwrap())(
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                start_angle,
                end_angle,
                color.into(),
            );
        }
    }

    /// Draws the bitmap scaled to xscale and yscale with its upper-left corner at location x, y. Note that flip is not available when drawing scaled bitmaps but negative scale values will achieve the same effect.
    pub fn draw_scaled_bitmap(&self, bitmap: impl AsRef<Bitmap>, pos: Vec2<i32>, scale: Vec2<f32>) {
        unsafe {
            ((*self.handle).drawScaledBitmap.unwrap())(
                bitmap.as_ref().handle,
                pos.x,
                pos.y,
                scale.x,
                scale.y,
            );
        }
    }

    /// Draws the given text using the provided options. If no font has been set with setFont, the default system font Asheville Sans 14 Light is used.
    pub fn draw_text(&self, text: impl AsRef<str>, pos: Vec2<i32>) -> i32 {
        let ptr = text.as_ref().as_ptr() as *const c_void;
        let len = text.as_ref().chars().count();
        unsafe {
            ((*self.handle).drawText.unwrap())(ptr, len, sys::PDStringEncoding::UTF8, pos.x, pos.y)
        }
    }

    /// Draws the text in the given rectangle using the provided options. If no font has been set with setFont, the default system font Asheville Sans 14 Light is used. See the above note about the len argument.
    pub fn draw_text_in_rect(
        &self,
        text: impl AsRef<str>,
        rect: Rect<i32>,
        wrap: TextWrap,
        align: TextAlign,
    ) {
        let ptr = text.as_ref().as_ptr() as *const c_void;
        let len = text.as_ref().chars().count();
        unsafe {
            ((*self.handle).drawTextInRect.unwrap())(
                ptr,
                len,
                sys::PDStringEncoding::UTF8,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                wrap,
                align,
            )
        }
    }

    /// Allocates and returns a new width by height LCDBitmap filled with bgcolor.
    pub fn new_bitmap(
        &self,
        width: i32,
        height: i32,
        bgcolor: impl Into<ColorOrPattern>,
    ) -> Bitmap {
        Bitmap::from(unsafe { ((*self.handle).newBitmap.unwrap())(width, height, bgcolor.into()) })
    }

    /// Allocates and returns a new LCDBitmap from the file at path. If there is no file at path, the function returns null.
    pub fn load_bitmap(&self, path: impl AsRef<str>) -> Result<Bitmap, Error> {
        unsafe {
            let c_string = CString::new(path.as_ref()).unwrap();
            let mut err: *const c_char = core::ptr::null();
            let ptr = ((*self.handle).loadBitmap.unwrap())(c_string.as_ptr() as _, &mut err);
            if !err.is_null() {
                let err = CString::from_raw(err as *mut c_char);
                let err = err.into_string().unwrap();
                return Err(Error::FailedToLoadBitMapFromFile(err));
            }
            Ok(Bitmap::from(ptr))
        }
    }

    /// Returns the current display frame buffer. Rows are 32-bit aligned, so the row stride is 52 bytes, with the extra 2 bytes per row ignored. Bytes are MSB-ordered; i.e., the pixel in column 0 is the 0x80 bit of the first byte of the row.
    pub fn get_frame(&self) -> *mut u8 {
        unsafe { ((*self.handle).getFrame.unwrap())() }
    }

    /// Returns the current display frame buffer. Rows are 32-bit aligned, so the row stride is 52 bytes, with the extra 2 bytes per row ignored. Bytes are MSB-ordered; i.e., the pixel in column 0 is the 0x80 bit of the first byte of the row.
    pub fn get_display_frame(&self) -> *mut u8 {
        unsafe { ((*self.handle).getDisplayFrame.unwrap())() }
    }

    /// Only valid in the Simulator, returns the debug framebuffer as a bitmap. Function is NULL on device.
    pub fn get_debug_bitmap(&self) -> Option<Ref<Bitmap>> {
        let ptr = unsafe { ((*self.handle).getDebugBitmap.unwrap())() };
        if ptr.is_null() {
            None
        } else {
            Some(Bitmap::from_ref(ptr))
        }
    }

    /// After updating pixels in the buffer returned by getFrame(), you must tell the graphics system which rows were updated. This function marks a contiguous range of rows as updated (e.g., markUpdatedRows(0,LCD_ROWS-1) tells the system to update the entire display). Both “start” and “end” are included in the range.
    pub fn mark_updated_rows(&self, start: i32, end: i32) {
        unsafe {
            ((*self.handle).markUpdatedRows.unwrap())(start, end);
        }
    }

    /// Manually flushes the current frame buffer out to the display. This function is automatically called after each pass through the run loop, so there shouldn’t be any need to call it yourself.
    pub fn display(&self) {
        unsafe {
            ((*self.handle).display.unwrap())();
        }
    }

    /// Sets the current clip rect in screen coordinates.
    pub fn set_screen_clip_rect(&self, rect: Rect<i32>) {
        unsafe {
            ((*self.handle).setScreenClipRect.unwrap())(rect.x, rect.y, rect.width, rect.height);
        }
    }

    /// Fills the polygon with vertices at the given coordinates (an array of 2*nPoints ints containing alternating x and y values) using the given color and fill, or winding, rule. See [Nonzero-rule](https://en.wikipedia.org/wiki/Nonzero-rule) for an explanation of the winding rule.
    pub fn fill_polygon(
        &self,
        n_points: i32,
        coords: impl AsRef<[i32]>,
        color: impl Into<ColorOrPattern>,
        fillrule: PolygonFillRule,
    ) {
        unsafe {
            let mut coords = coords.as_ref().to_vec();
            ((*self.handle).fillPolygon.unwrap())(
                n_points,
                coords.as_mut_ptr(),
                color.into(),
                fillrule,
            );
        }
    }

    /// Returns a bitmap containing the contents of the display buffer. The system owns this bitmap—​do not free it!
    pub fn get_display_buffer_bitmap(&self) -> Ref<Bitmap> {
        Bitmap::from_ref(unsafe { ((*self.handle).getDisplayBufferBitmap.unwrap())() })
    }

    /// Draws the bitmap scaled to xscale and yscale then rotated by degrees with its center as given by proportions centerx and centery at x, y; that is: if centerx and centery are both 0.5 the center of the image is at (x,y), if centerx and centery are both 0 the top left corner of the image (before rotation) is at (x,y), etc.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_rotated_bitmap(
        &self,
        bitmap: impl AsRef<Bitmap>,
        pos: Vec2<i32>,
        rotation: f32,
        center_pos: Vec2<f32>,
        scale: Vec2<f32>,
    ) {
        unsafe {
            ((*self.handle).drawRotatedBitmap.unwrap())(
                bitmap.as_ref().handle,
                pos.x,
                pos.y,
                rotation,
                center_pos.x,
                center_pos.y,
                scale.x,
                scale.y,
            );
        }
    }

    /// Sets the leading adjustment (added to the leading specified in the font) to use when drawing text.
    pub fn set_text_leading(&self, leading: i32) {
        unsafe {
            ((*self.handle).setTextLeading.unwrap())(leading);
        }
    }

    /// Sets the stencil used for drawing. If the tile flag is set the stencil image will be tiled. Tiled stencils must have width equal to a multiple of 32 pixels.
    pub fn set_stencil_image(&self, stencil: impl AsRef<Bitmap>, tile: i32) {
        unsafe {
            ((*self.handle).setStencilImage.unwrap())(stencil.as_ref().handle, tile);
        }
    }
}
