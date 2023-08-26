use core::{
    ffi::{c_char, c_void},
    marker::PhantomData,
};

use alloc::ffi::CString;

use crate::{math::SideOffsets2D, util::Ref};

pub use sys::{
    LCDBitmapDrawMode, LCDBitmapFlip, LCDColor, LCDFontData, LCDLineCapStyle, LCDPattern,
    LCDPolygonFillRule, LCDSolidColor, LCD_COLUMNS, LCD_ROWS, LCD_ROWSIZE,
};

pub(crate) fn so2d_to_lcdrect(r: SideOffsets2D<i32>) -> sys::LCDRect {
    sys::LCDRect {
        top: r.top,
        right: r.right,
        bottom: r.bottom,
        left: r.left,
    }
}

use crate::{error::Error, PLAYDATE};

pub struct PlaydateGraphics {
    handle: *const sys::playdate_graphics,
    pub video: crate::video::PlaydateVideo,
}

impl PlaydateGraphics {
    pub(crate) fn new(handle: *const sys::playdate_graphics) -> Self {
        Self {
            handle,
            video: crate::video::PlaydateVideo::new(unsafe { (*handle).video }),
        }
    }

    // pub video: *const playdate_video,

    /// Clears the entire display, filling it with color.
    pub fn clear(&self, color: impl Into<LCDColor>) {
        unsafe {
            ((*self.handle).clear.unwrap())(color.into());
        }
    }

    /// Sets the background color shown when the display is offset or for clearing dirty areas in the sprite system.
    pub fn set_background_color(&self, color: LCDSolidColor) {
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
    pub fn set_draw_mode(&self, mode: LCDBitmapDrawMode) {
        unsafe {
            ((*self.handle).setDrawMode.unwrap())(mode);
        }
    }

    /// Offsets the origin point for all drawing calls to x, y (can be negative).
    /// This is useful, for example, for centering a "camera" on a sprite that is moving around a world larger than the screen.
    pub fn set_draw_offset(&self, dx: i32, dy: i32) {
        unsafe {
            ((*self.handle).setDrawOffset.unwrap())(dx, dy);
        }
    }

    /// Sets the current clip rect, using world coordinates—​that is, the given rectangle will be translated by the current drawing offset. The clip rect is cleared at the beginning of each update.
    pub fn set_clip_rect(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            ((*self.handle).setClipRect.unwrap())(x, y, width, height);
        }
    }

    /// Clears the current clip rect.
    pub fn clear_clip_rect(&self) {
        unsafe {
            ((*self.handle).clearClipRect.unwrap())();
        }
    }

    /// Sets the end cap style used in the line drawing functions.
    pub fn set_line_cap_style(&self, end_cap_style: LCDLineCapStyle) {
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
    pub fn draw_bitmap(&self, bitmap: impl AsRef<Bitmap>, x: i32, y: i32, flip: LCDBitmapFlip) {
        unsafe {
            ((*self.handle).drawBitmap.unwrap())(bitmap.as_ref().handle, x, y, flip);
        }
    }

    /// Draws the bitmap with its upper-left corner at location x, y tiled inside a width by height rectangle.
    pub fn tile_bitmap(
        &self,
        bitmap: impl AsRef<Bitmap>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        flip: LCDBitmapFlip,
    ) {
        unsafe {
            ((*self.handle).tileBitmap.unwrap())(bitmap.as_ref().handle, x, y, width, height, flip);
        }
    }

    /// Draws a line from x1, y1 to x2, y2 with a stroke width of width.
    pub fn draw_line(
        &self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        width: i32,
        color: impl Into<LCDColor>,
    ) {
        unsafe {
            ((*self.handle).drawLine.unwrap())(x1, y1, x2, y2, width, color.into());
        }
    }

    /// Draws a filled triangle with points at x1, y1, x2, y2, and x3, y3.
    #[allow(clippy::too_many_arguments)]
    pub fn fill_triangle(
        &self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        color: impl Into<LCDColor>,
    ) {
        unsafe {
            ((*self.handle).fillTriangle.unwrap())(x1, y1, x2, y2, x3, y3, color.into());
        }
    }

    /// Draws a pixel at x, y.
    pub fn draw_pixel(&self, x: i32, y: i32, color: LCDSolidColor) {
        let fb = self.get_frame();
        let byte_ptr = unsafe { fb.add((y * LCD_ROWSIZE as i32 + (x >> 3)) as usize) };
        if color == LCDSolidColor::kColorBlack {
            unsafe { *byte_ptr &= !(1 << (7 - (x & 7))) };
        } else {
            unsafe { *byte_ptr |= 1 << (7 - (x & 7)) };
        }
    }

    /// Draws a width by height rect at x, y.
    pub fn draw_rect(&self, x: i32, y: i32, width: i32, height: i32, color: impl Into<LCDColor>) {
        unsafe {
            ((*self.handle).drawRect.unwrap())(x, y, width, height, color.into());
        }
    }

    /// Draws a filled width by height rect at x, y.
    pub fn fill_rect(&self, x: i32, y: i32, width: i32, height: i32, color: impl Into<LCDColor>) {
        unsafe {
            ((*self.handle).fillRect.unwrap())(x, y, width, height, color.into());
        }
    }

    /// Draws an ellipse inside the rectangle {x, y, width, height} of width lineWidth (inset from the rectangle bounds). If startAngle != _endAngle, this draws an arc between the given angles. Angles are given in degrees, clockwise from due north.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_ellipse(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        line_width: i32,
        start_angle: f32,
        end_angle: f32,
        color: impl Into<LCDColor>,
    ) {
        unsafe {
            ((*self.handle).drawEllipse.unwrap())(
                x,
                y,
                width,
                height,
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
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        start_angle: f32,
        end_angle: f32,
        color: impl Into<LCDColor>,
    ) {
        unsafe {
            ((*self.handle).fillEllipse.unwrap())(
                x,
                y,
                width,
                height,
                start_angle,
                end_angle,
                color.into(),
            );
        }
    }

    /// Draws the bitmap scaled to xscale and yscale with its upper-left corner at location x, y. Note that flip is not available when drawing scaled bitmaps but negative scale values will achieve the same effect.
    pub fn draw_scaled_bitmap(
        &self,
        bitmap: impl AsRef<Bitmap>,
        x: i32,
        y: i32,
        xscale: f32,
        yscale: f32,
    ) {
        unsafe {
            ((*self.handle).drawScaledBitmap.unwrap())(
                bitmap.as_ref().handle,
                x,
                y,
                xscale,
                yscale,
            );
        }
    }

    /// Draws the given text using the provided options. If no font has been set with setFont, the default system font Asheville Sans 14 Light is used.
    pub fn draw_text(&self, text: impl AsRef<str>, x: i32, y: i32) -> i32 {
        let ptr = text.as_ref().as_ptr() as *const c_void;
        let len = text.as_ref().len();
        unsafe {
            ((*self.handle).drawText.unwrap())(ptr, len, sys::PDStringEncoding::kUTF8Encoding, x, y)
        }
    }

    /// Allocates and returns a new width by height LCDBitmap filled with bgcolor.
    pub fn new_bitmap(&self, width: i32, height: i32, bgcolor: impl Into<LCDColor>) -> Bitmap {
        Bitmap::from(unsafe { ((*self.handle).newBitmap.unwrap())(width, height, bgcolor.into()) })
    }

    /// Frees the given bitmap.
    pub(crate) fn free_bitmap(&self, bitmap: *mut sys::LCDBitmap) {
        unsafe {
            ((*self.handle).freeBitmap.unwrap())(bitmap);
        }
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

    /// Returns a new LCDBitmap that is an exact copy of bitmap.
    pub(crate) fn copy_bitmap(&self, bitmap: *mut sys::LCDBitmap) -> *mut sys::LCDBitmap {
        unsafe { ((*self.handle).copyBitmap.unwrap())(bitmap) }
    }

    /// Loads the image at path into the previously allocated bitmap.
    pub(crate) fn load_into_bitmap(
        &self,
        path: impl AsRef<str>,
        bitmap: *mut sys::LCDBitmap,
        outerr: *mut *const c_char,
    ) {
        unsafe {
            let c_string = CString::new(path.as_ref()).unwrap();
            ((*self.handle).loadIntoBitmap.unwrap())(c_string.as_ptr() as _, bitmap, outerr)
        }
    }

    /// Gets various info about bitmap including its width and height and raw pixel data. The data is 1 bit per pixel packed format, in MSB order; in other words, the high bit of the first byte in data is the top left pixel of the image. If the bitmap has a mask, a pointer to its data is returned in mask, else NULL is returned.
    pub(crate) fn get_bitmap_data(
        &self,
        bitmap: *mut sys::LCDBitmap,
        width: *mut i32,
        height: *mut i32,
        rowbytes: *mut i32,
        mask: *mut *mut u8,
        data: *mut *mut u8,
    ) {
        unsafe {
            ((*self.handle).getBitmapData.unwrap())(bitmap, width, height, rowbytes, mask, data)
        }
    }

    /// Clears bitmap, filling with the given bgcolor.
    pub(crate) fn clear_bitmap(&self, bitmap: *mut sys::LCDBitmap, bgcolor: impl Into<LCDColor>) {
        unsafe {
            ((*self.handle).clearBitmap.unwrap())(bitmap, bgcolor.into());
        }
    }

    /// Returns a new, rotated and scaled LCDBitmap based on the given bitmap.
    pub(crate) fn rotated_bitmap(
        &self,
        bitmap: *mut sys::LCDBitmap,
        rotation: f32,
        xscale: f32,
        yscale: f32,
        alloced_size: *mut i32,
    ) -> *mut sys::LCDBitmap {
        unsafe {
            ((*self.handle).rotatedBitmap.unwrap())(bitmap, rotation, xscale, yscale, alloced_size)
        }
    }

    /// Allocates and returns a new LCDBitmapTable that can hold count width by height LCDBitmaps.
    pub fn new_bitmap_table(&self, count: i32, width: i32, height: i32) -> BitmapTable {
        BitmapTable::from(unsafe { ((*self.handle).newBitmapTable.unwrap())(count, width, height) })
    }

    pub(crate) fn free_bitmap_table(&self, table: *mut sys::LCDBitmapTable) {
        unsafe {
            ((*self.handle).freeBitmapTable.unwrap())(table);
        }
    }

    /// Allocates and returns a new LCDBitmap from the file at path. If there is no file at path, the function returns null.
    pub fn load_bitmap_table(&self, path: impl AsRef<str>) -> Result<BitmapTable, Error> {
        unsafe {
            let c_string = CString::new(path.as_ref()).unwrap();
            let mut err = core::ptr::null();
            let ptr = ((*self.handle).loadBitmapTable.unwrap())(c_string.as_ptr() as _, &mut err);
            if !err.is_null() {
                let err = CString::from_raw(err as *mut c_char);
                let err = err.into_string().unwrap();
                return Err(Error::FailedToLoadBitMapTableFromFile(err));
            }
            Ok(BitmapTable::from(ptr))
        }
    }

    /// Allocates and returns a new LCDBitmap from the file at path. If there is no file at path, the function returns null.
    pub(crate) fn load_into_bitmap_table(
        &self,
        path: impl AsRef<str>,
        table: *mut sys::LCDBitmapTable,
        outerr: *mut *const c_char,
    ) {
        unsafe {
            let c_string = CString::new(path.as_ref()).unwrap();
            ((*self.handle).loadIntoBitmapTable.unwrap())(c_string.as_ptr() as _, table, outerr)
        }
    }

    /// Returns the idx bitmap in table, If idx is out of bounds, the function returns NULL.
    pub(crate) fn get_table_bitmap(
        &self,
        table: *mut sys::LCDBitmapTable,
        idx: i32,
    ) -> *mut sys::LCDBitmap {
        unsafe { ((*self.handle).getTableBitmap.unwrap())(table, idx) }
    }

    /// Returns the LCDFont object for the font file at path. In case of error, outErr points to a string describing the error.
    pub fn load_font(&self, path: impl AsRef<str>) -> Result<Font, Error> {
        unsafe {
            let c_string = CString::new(path.as_ref()).unwrap();
            let mut err = core::ptr::null();
            let font = ((*self.handle).loadFont.unwrap())(c_string.as_ptr() as _, &mut err);
            if !err.is_null() {
                let err = CString::from_raw(err as *mut c_char);
                let err = err.into_string().unwrap();
                return Err(Error::FailedToLoadFont(err));
            }
            Ok(Font::new(font))
        }
    }

    /// Returns an LCDFontPage object for the given character code. Each LCDFontPage contains information for 256 characters; specifically, if (c1 & ~0xff) == (c2 & ~0xff), then c1 and c2 belong to the same page and the same LCDFontPage can be used to fetch the character data for both instead of searching for the page twice.
    pub(crate) fn get_font_page(&self, font: *mut sys::LCDFont, c: u32) -> *mut sys::LCDFontPage {
        unsafe { ((*self.handle).getFontPage.unwrap())(font, c) }
    }

    /// Returns an LCDFontGlyph object for character c in LCDFontPage page, and optionally returns the glyph’s bitmap and advance value.
    pub(crate) fn get_page_glyph(
        &self,
        page: *mut sys::LCDFontPage,
        c: u32,
        bitmap: *mut *mut sys::LCDBitmap,
        advance: *mut i32,
    ) -> *mut sys::LCDFontGlyph {
        unsafe { ((*self.handle).getPageGlyph.unwrap())(page, c, bitmap, advance) }
    }

    /// Returns the kerning adjustment between characters c1 and c2 as specified by the font.
    pub(crate) fn get_glyph_kerning(
        &self,
        glyph: *mut sys::LCDFontGlyph,
        glyphcode: u32,
        nextcode: u32,
    ) -> i32 {
        unsafe { ((*self.handle).getGlyphKerning.unwrap())(glyph, glyphcode, nextcode) }
    }

    /// Returns the width of the given text in the given font.
    pub(crate) fn get_text_width(
        &self,
        font: *mut sys::LCDFont,
        text: impl AsRef<str>,
        tracking: i32,
    ) -> i32 {
        let ptr = text.as_ref().as_ptr() as *const c_void;
        let len = text.as_ref().len();
        unsafe {
            ((*self.handle).getTextWidth.unwrap())(
                font,
                ptr,
                len,
                sys::PDStringEncoding::kUTF8Encoding,
                tracking,
            )
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

    /// Returns a copy the contents of the working frame buffer as a bitmap. The caller is responsible for freeing the returned bitmap with playdate->graphics->freeBitmap().
    pub fn copy_frame_buffer_bitmap(&self) -> Bitmap {
        Bitmap::from(unsafe { ((*self.handle).copyFrameBufferBitmap.unwrap())() })
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

    /// Sets color to an 8 x 8 pattern using the given bitmap. x, y indicates the top left corner of the 8 x 8 pattern.
    pub(crate) fn set_color_to_pattern(
        &self,
        color: *mut LCDColor,
        bitmap: *mut sys::LCDBitmap,
        x: i32,
        y: i32,
    ) {
        unsafe {
            ((*self.handle).setColorToPattern.unwrap())(color, bitmap, x, y);
        }
    }

    /// Returns 1 if any of the opaque pixels in bitmap1 when positioned at x1, y1 with flip1 overlap any of the opaque pixels in bitmap2 at x2, y2 with flip2 within the non-empty rect, or 0 if no pixels overlap or if one or both fall completely outside of rect.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn check_mask_collision(
        &self,
        bitmap1: *mut sys::LCDBitmap,
        x1: i32,
        y1: i32,
        flip1: LCDBitmapFlip,
        bitmap2: *mut sys::LCDBitmap,
        x2: i32,
        y2: i32,
        flip2: LCDBitmapFlip,
        rect: SideOffsets2D<i32>,
    ) -> i32 {
        unsafe {
            ((*self.handle).checkMaskCollision.unwrap())(
                bitmap1,
                x1,
                y1,
                flip1,
                bitmap2,
                x2,
                y2,
                flip2,
                so2d_to_lcdrect(rect),
            )
        }
    }

    /// Sets the current clip rect in screen coordinates.
    pub fn set_screen_clip_rect(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            ((*self.handle).setScreenClipRect.unwrap())(x, y, width, height);
        }
    }

    /// Fills the polygon with vertices at the given coordinates (an array of 2*nPoints ints containing alternating x and y values) using the given color and fill, or winding, rule. See [Nonzero-rule](https://en.wikipedia.org/wiki/Nonzero-rule) for an explanation of the winding rule.
    pub fn fill_polygon(
        &self,
        n_points: i32,
        coords: impl AsRef<[i32]>,
        color: impl Into<LCDColor>,
        fillrule: LCDPolygonFillRule,
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

    /// Returns the height of the given font.
    pub(crate) fn get_font_height(&self, font: *mut sys::LCDFont) -> u8 {
        unsafe { ((*self.handle).getFontHeight.unwrap())(font) }
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
        x: i32,
        y: i32,
        rotation: f32,
        centerx: f32,
        centery: f32,
        xscale: f32,
        yscale: f32,
    ) {
        unsafe {
            ((*self.handle).drawRotatedBitmap.unwrap())(
                bitmap.as_ref().handle,
                x,
                y,
                rotation,
                centerx,
                centery,
                xscale,
                yscale,
            );
        }
    }

    /// Sets the leading adjustment (added to the leading specified in the font) to use when drawing text.
    pub fn set_text_leading(&self, leading: i32) {
        unsafe {
            ((*self.handle).setTextLeading.unwrap())(leading);
        }
    }

    /// Sets a mask image for the given bitmap. The set mask must be the same size as the target bitmap.
    pub(crate) fn set_bitmap_mask(
        &self,
        bitmap: *mut sys::LCDBitmap,
        mask: *mut sys::LCDBitmap,
    ) -> i32 {
        unsafe { ((*self.handle).setBitmapMask.unwrap())(bitmap, mask) }
    }

    /// Gets a mask image for the given bitmap. If the image doesn’t have a mask, getBitmapMask returns NULL.
    pub(crate) fn get_bitmap_mask(&self, bitmap: *mut sys::LCDBitmap) -> Ref<Bitmap> {
        Bitmap::from_ref(unsafe { ((*self.handle).getBitmapMask.unwrap())(bitmap) })
    }

    /// Sets the stencil used for drawing. If the tile flag is set the stencil image will be tiled. Tiled stencils must have width equal to a multiple of 32 pixels.
    pub fn set_stencil_image(&self, stencil: impl AsRef<Bitmap>, tile: i32) {
        unsafe {
            ((*self.handle).setStencilImage.unwrap())(stencil.as_ref().handle, tile);
        }
    }

    /// Returns an LCDFont object wrapping the LCDFontData data comprising the contents (minus 16-byte header) of an uncompressed pft file. wide corresponds to the flag in the header indicating whether the font contains glyphs at codepoints above U+1FFFF.
    /// # Safety
    /// Assumes that the LCDFontData is valid.
    pub unsafe fn make_font_from_data(&self, data: *mut sys::LCDFontData, wide: i32) -> Font {
        Font::new(((*self.handle).makeFontFromData.unwrap())(data, wide))
    }
}

/// A bitmap instance with ownership to the underlying data.
#[derive(Debug)]
pub struct Bitmap {
    pub(crate) handle: *mut sys::LCDBitmap,
}

impl Bitmap {
    pub(crate) fn from(handle: *mut sys::LCDBitmap) -> Self {
        Self { handle }
    }

    pub(crate) fn from_ref<'a>(handle: *mut sys::LCDBitmap) -> Ref<'a, Self> {
        Ref::new(Self { handle })
    }

    /// Allocates and returns a new width by height Bitmap filled with bgcolor.
    pub fn new(width: i32, height: i32, bgcolor: impl Into<LCDColor>) -> Self {
        PLAYDATE.graphics.new_bitmap(width, height, bgcolor)
    }

    /// Clears bitmap, filling with the given bgcolor.
    pub fn clear(&self) {
        PLAYDATE
            .graphics
            .clear_bitmap(self.handle, LCDSolidColor::kColorWhite);
    }

    /// Sets a mask image for the given bitmap. The set mask must be the same size as the target bitmap.
    pub fn set_mask(&self, mask: impl AsRef<Bitmap>) -> Result<(), Error> {
        let result = -PLAYDATE
            .graphics
            .set_bitmap_mask(self.handle, mask.as_ref().handle);
        if result != 1 {
            Err(Error::FailedToSetBitmapMask)
        } else {
            Ok(())
        }
    }

    /// Gets a mask image for the given bitmap. If the image doesn’t have a mask, getBitmapMask returns NULL.
    pub fn get_mask(&self) -> Ref<Bitmap> {
        PLAYDATE.graphics.get_bitmap_mask(self.handle)
    }

    /// Returns `true` if any of the opaque pixels in `self` when positioned at `x1`, `y1` with `flip1` overlap any of the opaque pixels in `other` at `x2`, `y2` with `flip2` within the non-empty rect, or `false` if no pixels overlap or if one or both fall completely outside of rect.
    #[allow(clippy::too_many_arguments)]
    pub fn check_mask_collision(
        &self,
        x1: i32,
        y1: i32,
        flip1: LCDBitmapFlip,
        other: impl AsRef<Bitmap>,
        x2: i32,
        y2: i32,
        flip2: LCDBitmapFlip,
        rect: SideOffsets2D<i32>,
    ) -> bool {
        PLAYDATE.graphics.check_mask_collision(
            self.handle,
            x1,
            y1,
            flip1,
            other.as_ref().handle,
            x2,
            y2,
            flip2,
            rect,
        ) == 1
    }

    /// Gets various info about bitmap including its width and height and raw pixel data. The data is 1 bit per pixel packed format, in MSB order; in other words, the high bit of the first byte in data is the top left pixel of the image. If the bitmap has a mask, a pointer to its data is returned in mask, else NULL is returned.
    pub fn get_bitmap_data(&self) -> BitmapData {
        let mut data = BitmapData::new();
        PLAYDATE.graphics.get_bitmap_data(
            self.handle,
            &mut data.width,
            &mut data.height,
            &mut data.rowbytes,
            &mut data.mask,
            &mut data.data,
        );
        data
    }

    /// Loads the image at path into the previously allocated bitmap.
    pub fn load(&self, path: impl AsRef<str>) -> Result<(), Error> {
        let mut err: *const c_char = core::ptr::null();
        PLAYDATE
            .graphics
            .load_into_bitmap(path, self.handle, &mut err);
        if !err.is_null() {
            let err = unsafe { CString::from_raw(err as *mut c_char) };
            let err = err.into_string().unwrap();
            return Err(Error::FailedToLoadBitMapFromFile(err));
        }
        Ok(())
    }

    /// Returns a new, rotated and scaled LCDBitmap based on the given bitmap.
    pub fn rotated(&self, rotation: f32, xscale: f32, yscale: f32) -> Bitmap {
        let mut alloced_size = 0;
        Bitmap::from(PLAYDATE.graphics.rotated_bitmap(
            self.handle,
            rotation,
            xscale,
            yscale,
            &mut alloced_size,
        ))
    }

    /// Get color as an 8 x 8 pattern using the given bitmap. x, y indicates the top left corner of the 8 x 8 pattern.
    pub fn get_color_pattern(&self, x: i32, y: i32) -> ColorPatternData {
        let mut color = LCDColor::default();
        PLAYDATE
            .graphics
            .set_color_to_pattern(&mut color, self.handle, x, y);
        if let Some(scolor) = color.as_solid_color() {
            ColorPatternData::Solid(scolor)
        } else {
            ColorPatternData::Pattern(unsafe { color.as_pattern().unwrap() })
        }
    }
}

impl AsRef<Self> for Bitmap {
    fn as_ref(&self) -> &Self {
        self
    }
}

unsafe impl Send for Bitmap {}
unsafe impl Sync for Bitmap {}

impl PartialEq for Bitmap {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for Bitmap {}

impl Drop for Bitmap {
    fn drop(&mut self) {
        PLAYDATE.graphics.free_bitmap(self.handle);
    }
}

impl Clone for Bitmap {
    fn clone(&self) -> Self {
        Bitmap {
            handle: PLAYDATE.graphics.copy_bitmap(self.handle),
        }
    }
}

pub enum ColorPatternData {
    Solid(LCDSolidColor),
    Pattern(LCDPattern),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BitmapData<'a> {
    pub width: i32,
    pub height: i32,
    pub rowbytes: i32,
    pub mask: *mut u8,
    pub data: *mut u8,
    _p: PhantomData<&'a ()>,
}

impl<'a> BitmapData<'a> {
    fn new() -> Self {
        BitmapData {
            width: 0,
            height: 0,
            rowbytes: 0,
            mask: core::ptr::null_mut(),
            data: core::ptr::null_mut(),
            _p: PhantomData,
        }
    }
}

/// There are two kinds of image tables: matrix and sequential.
///
/// Matrix image tables are great as sources of imagery for tilemap. They are loaded from a single file in your game’s source folder with the suffix -table-<w>-<h> before the file extension. The compiler splits the image into separate bitmaps of dimension w by h pixels that are accessible via imagetable:getImage(x,y).
///
/// Sequential image tables are useful as a way to load up sequential frames of animation. They are loaded from a sequence of files in your game’s source folder at compile time from filenames with the suffix -table-<sequenceNumber> before the file extension. Individual images in the sequence are accessible via imagetable:getImage(n). The images employed by a sequential image table are not required to be the same size, unlike the images used in a matrix image table.
#[derive(PartialEq, Eq, Debug)]
pub struct BitmapTable {
    handle: *mut sys::LCDBitmapTable,
}

unsafe impl Send for BitmapTable {}
unsafe impl Sync for BitmapTable {}

impl BitmapTable {
    fn from(handle: *mut sys::LCDBitmapTable) -> Self {
        Self { handle }
    }

    pub fn new(count: usize, width: i32, height: i32) -> Self {
        PLAYDATE
            .graphics
            .new_bitmap_table(count as _, width, height)
    }

    /// Returns the idx bitmap in table, If idx is out of bounds, the function returns NULL.
    pub fn get(&self, idx: usize) -> Option<Ref<Bitmap>> {
        let ptr = PLAYDATE.graphics.get_table_bitmap(self.handle, idx as _);
        if ptr.is_null() {
            return None;
        }
        Some(Bitmap::from_ref(ptr))
    }

    /// Allocates and returns a new LCDBitmap from the file at path. If there is no file at path, the function returns null.
    pub fn load(&self, path: impl AsRef<str>) -> Result<(), Error> {
        let mut err: *const c_char = core::ptr::null();
        PLAYDATE
            .graphics
            .load_into_bitmap_table(path, self.handle, &mut err);
        if !err.is_null() {
            let err = unsafe { CString::from_raw(err as *mut c_char) };
            let err = err.into_string().unwrap();
            return Err(Error::FailedToLoadBitMapFromBitMapTable(err));
        }
        Ok(())
    }
}

impl Drop for BitmapTable {
    fn drop(&mut self) {
        PLAYDATE.graphics.free_bitmap_table(self.handle);
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Font {
    handle: *mut sys::LCDFont,
}

unsafe impl Send for Font {}
unsafe impl Sync for Font {}

impl Font {
    fn new(handle: *mut sys::LCDFont) -> Self {
        Self { handle }
    }

    /// Returns the height of the given font.
    pub fn get_height(&self) -> u8 {
        PLAYDATE.graphics.get_font_height(self.handle)
    }

    /// Returns the width of the given text in the given font.
    pub fn get_text_width(&self, text: impl AsRef<str>, tracking: i32) -> i32 {
        PLAYDATE
            .graphics
            .get_text_width(self.handle, text, tracking)
    }

    /// Returns an LCDFontPage object for the given character code. Each LCDFontPage contains information for 256 characters; specifically, if (c1 & ~0xff) == (c2 & ~0xff), then c1 and c2 belong to the same page and the same LCDFontPage can be used to fetch the character data for both instead of searching for the page twice.
    pub fn get_page(&self, c: u32) -> FontPage {
        FontPage::new(PLAYDATE.graphics.get_font_page(self.handle, c))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct FontPage {
    handle: *mut sys::LCDFontPage,
}

unsafe impl Send for FontPage {}
unsafe impl Sync for FontPage {}

impl FontPage {
    fn new(handle: *mut sys::LCDFontPage) -> Self {
        Self { handle }
    }

    /// Returns an LCDFontGlyph object for character c in LCDFontPage page, and optionally returns the glyph’s bitmap and advance value.
    pub fn get_glyph(&self, c: u32) -> (FontGlyph, Option<Ref<Bitmap>>, Option<i32>) {
        let mut bitmap = core::ptr::null_mut();
        let mut advance = 0;
        let glyph = FontGlyph::new(PLAYDATE.graphics.get_page_glyph(
            self.handle,
            c,
            &mut bitmap,
            &mut advance,
        ));
        let bitmap = if bitmap.is_null() {
            None
        } else {
            Some(Bitmap::from_ref(bitmap))
        };
        let advance = if advance == 0 { None } else { Some(advance) };
        (glyph, bitmap, advance)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct FontGlyph {
    handle: *mut sys::LCDFontGlyph,
}

unsafe impl Send for FontGlyph {}
unsafe impl Sync for FontGlyph {}

impl FontGlyph {
    fn new(handle: *mut sys::LCDFontGlyph) -> Self {
        Self { handle }
    }

    /// Returns the kerning adjustment between characters c1 and c2 as specified by the font.
    pub fn get_kerning(&self, c1: u32, c2: u32) -> i32 {
        PLAYDATE.graphics.get_glyph_kerning(self.handle, c1, c2)
    }
}
