use core::ffi::{c_char, c_void};

use alloc::ffi::CString;

pub use sys::{
    LCDBitmap, LCDBitmapDrawMode, LCDBitmapFlip, LCDBitmapTable, LCDColor, LCDFont, LCDFontData,
    LCDFontGlyph, LCDFontPage, LCDLineCapStyle, LCDPolygonFillRule, LCDRect, LCDSolidColor,
    LCD_COLUMNS, LCD_ROWS, LCD_ROWSIZE,
};

pub struct Graphics {
    handle: *const sys::playdate_graphics,
}

impl Graphics {
    pub(crate) fn new(handle: *const sys::playdate_graphics) -> Self {
        Graphics { handle }
    }

    // pub video: *const playdate_video,

    pub fn clear(&self, color: LCDColor) {
        unsafe {
            ((*self.handle).clear.unwrap())(color);
        }
    }

    pub fn set_background_color(&self, color: LCDSolidColor) {
        unsafe {
            ((*self.handle).setBackgroundColor.unwrap())(color);
        }
    }

    pub fn set_stencil(&self, stencil: *mut LCDBitmap) {
        unsafe {
            ((*self.handle).setStencil.unwrap())(stencil);
        }
    }

    pub fn set_draw_mode(&self, mode: LCDBitmapDrawMode) {
        unsafe {
            ((*self.handle).setDrawMode.unwrap())(mode);
        }
    }

    pub fn set_draw_offset(&self, dx: i32, dy: i32) {
        unsafe {
            ((*self.handle).setDrawOffset.unwrap())(dx, dy);
        }
    }

    pub fn set_clip_rect(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            ((*self.handle).setClipRect.unwrap())(x, y, width, height);
        }
    }

    pub fn clear_clip_rect(&self) {
        unsafe {
            ((*self.handle).clearClipRect.unwrap())();
        }
    }

    pub fn set_line_cap_style(&self, end_cap_style: LCDLineCapStyle) {
        unsafe {
            ((*self.handle).setLineCapStyle.unwrap())(end_cap_style);
        }
    }

    pub fn set_font(&self, font: *mut LCDFont) {
        unsafe {
            ((*self.handle).setFont.unwrap())(font);
        }
    }

    pub fn set_text_tracking(&self, tracking: i32) {
        unsafe {
            ((*self.handle).setTextTracking.unwrap())(tracking);
        }
    }

    pub fn push_context(&self, target: *mut LCDBitmap) {
        unsafe {
            ((*self.handle).pushContext.unwrap())(target);
        }
    }

    pub fn pop_context(&self) {
        unsafe {
            ((*self.handle).popContext.unwrap())();
        }
    }

    pub fn draw_bitmap(&self, bitmap: *mut LCDBitmap, x: i32, y: i32, flip: LCDBitmapFlip) {
        unsafe {
            ((*self.handle).drawBitmap.unwrap())(bitmap, x, y, flip);
        }
    }

    pub fn tile_bitmap(
        &self,
        bitmap: *mut LCDBitmap,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        flip: LCDBitmapFlip,
    ) {
        unsafe {
            ((*self.handle).tileBitmap.unwrap())(bitmap, x, y, width, height, flip);
        }
    }

    pub fn draw_line(&self, x1: i32, y1: i32, x2: i32, y2: i32, width: i32, color: LCDColor) {
        unsafe {
            ((*self.handle).drawLine.unwrap())(x1, y1, x2, y2, width, color);
        }
    }

    pub fn fill_triangle(
        &self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        color: LCDColor,
    ) {
        unsafe {
            ((*self.handle).fillTriangle.unwrap())(x1, y1, x2, y2, x3, y3, color);
        }
    }

    pub fn draw_rect(&self, x: i32, y: i32, width: i32, height: i32, color: LCDColor) {
        unsafe {
            ((*self.handle).drawRect.unwrap())(x, y, width, height, color);
        }
    }

    pub fn fill_rect(&self, x: i32, y: i32, width: i32, height: i32, color: LCDColor) {
        unsafe {
            ((*self.handle).fillRect.unwrap())(x, y, width, height, color);
        }
    }

    pub fn draw_ellipse(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        line_width: i32,
        start_angle: f32,
        end_angle: f32,
        color: LCDColor,
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
                color,
            );
        }
    }

    pub fn fill_ellipse(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        start_angle: f32,
        end_angle: f32,
        color: LCDColor,
    ) {
        unsafe {
            ((*self.handle).fillEllipse.unwrap())(
                x,
                y,
                width,
                height,
                start_angle,
                end_angle,
                color,
            );
        }
    }

    pub fn draw_scaled_bitmap(
        &self,
        bitmap: *mut LCDBitmap,
        x: i32,
        y: i32,
        xscale: f32,
        yscale: f32,
    ) {
        unsafe {
            ((*self.handle).drawScaledBitmap.unwrap())(bitmap, x, y, xscale, yscale);
        }
    }

    pub fn draw_text(&self, text: impl AsRef<str>, x: i32, y: i32) -> i32 {
        let ptr = text.as_ref().as_ptr() as *const c_void;
        let len = text.as_ref().len();
        unsafe {
            ((*self.handle).drawText.unwrap())(ptr, len, sys::PDStringEncoding::kUTF8Encoding, x, y)
        }
    }

    pub fn new_bitmap(&self, width: i32, height: i32, bgcolor: LCDColor) -> *mut LCDBitmap {
        unsafe { ((*self.handle).newBitmap.unwrap())(width, height, bgcolor) }
    }

    pub fn free_bitmap(&self, bitmap: *mut LCDBitmap) {
        unsafe {
            ((*self.handle).freeBitmap.unwrap())(bitmap);
        }
    }

    pub fn load_bitmap(&self, path: impl AsRef<str>, outerr: *mut *const c_char) -> *mut LCDBitmap {
        unsafe {
            let c_string = CString::new(path.as_ref()).unwrap();
            ((*self.handle).loadBitmap.unwrap())(c_string.as_ptr() as _, outerr)
        }
    }

    pub fn copy_bitmap(&self, bitmap: *mut LCDBitmap) -> *mut LCDBitmap {
        unsafe { ((*self.handle).copyBitmap.unwrap())(bitmap) }
    }

    pub fn load_into_bitmap(
        &self,
        path: impl AsRef<str>,
        bitmap: *mut LCDBitmap,
        outerr: *mut *const c_char,
    ) {
        unsafe {
            let c_string = CString::new(path.as_ref()).unwrap();
            ((*self.handle).loadIntoBitmap.unwrap())(c_string.as_ptr() as _, bitmap, outerr)
        }
    }

    pub fn get_bitmap_data(
        &self,
        bitmap: *mut LCDBitmap,
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

    pub fn clear_bitmap(&self, bitmap: *mut LCDBitmap, bgcolor: LCDColor) {
        unsafe {
            ((*self.handle).clearBitmap.unwrap())(bitmap, bgcolor);
        }
    }

    pub fn rotated_bitmap(
        &self,
        bitmap: *mut LCDBitmap,
        rotation: f32,
        xscale: f32,
        yscale: f32,
        alloced_size: *mut i32,
    ) -> *mut LCDBitmap {
        unsafe {
            ((*self.handle).rotatedBitmap.unwrap())(bitmap, rotation, xscale, yscale, alloced_size)
        }
    }

    pub fn new_bitmap_table(&self, count: i32, width: i32, height: i32) -> *mut LCDBitmapTable {
        unsafe { ((*self.handle).newBitmapTable.unwrap())(count, width, height) }
    }

    pub fn free_bitmap_table(&self, table: *mut LCDBitmapTable) {
        unsafe {
            ((*self.handle).freeBitmapTable.unwrap())(table);
        }
    }

    pub fn load_bitmap_table(
        &self,
        path: impl AsRef<str>,
        outerr: *mut *const c_char,
    ) -> *mut LCDBitmapTable {
        unsafe {
            let c_string = CString::new(path.as_ref()).unwrap();
            ((*self.handle).loadBitmapTable.unwrap())(c_string.as_ptr() as _, outerr)
        }
    }

    pub fn load_into_bitmap_table(
        &self,
        path: impl AsRef<str>,
        table: *mut LCDBitmapTable,
        outerr: *mut *const c_char,
    ) {
        unsafe {
            let c_string = CString::new(path.as_ref()).unwrap();
            ((*self.handle).loadIntoBitmapTable.unwrap())(c_string.as_ptr() as _, table, outerr)
        }
    }

    pub fn get_table_bitmap(&self, table: *mut LCDBitmapTable, idx: i32) -> *mut LCDBitmap {
        unsafe { ((*self.handle).getTableBitmap.unwrap())(table, idx) }
    }

    pub fn load_font(&self, path: impl AsRef<str>, outerr: *mut *const c_char) -> *mut LCDFont {
        unsafe {
            let c_string = CString::new(path.as_ref()).unwrap();
            ((*self.handle).loadFont.unwrap())(c_string.as_ptr() as _, outerr)
        }
    }

    pub fn get_font_page(&self, font: *mut LCDFont, c: u32) -> *mut LCDFontPage {
        unsafe { ((*self.handle).getFontPage.unwrap())(font, c) }
    }

    pub fn get_page_glyph(
        &self,
        page: *mut LCDFontPage,
        c: u32,
        bitmap: *mut *mut LCDBitmap,
        advance: *mut i32,
    ) -> *mut LCDFontGlyph {
        unsafe { ((*self.handle).getPageGlyph.unwrap())(page, c, bitmap, advance) }
    }

    pub fn get_glyph_kerning(
        &self,
        glyph: *mut LCDFontGlyph,
        glyphcode: u32,
        nextcode: u32,
    ) -> i32 {
        unsafe { ((*self.handle).getGlyphKerning.unwrap())(glyph, glyphcode, nextcode) }
    }

    pub fn get_text_width(&self, font: *mut LCDFont, text: impl AsRef<str>, tracking: i32) -> i32 {
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

    pub fn get_frame(&self) -> *mut u8 {
        unsafe { ((*self.handle).getFrame.unwrap())() }
    }

    pub fn get_display_frame(&self) -> *mut u8 {
        unsafe { ((*self.handle).getDisplayFrame.unwrap())() }
    }

    pub fn get_debug_bitmap(&self) -> *mut LCDBitmap {
        unsafe { ((*self.handle).getDebugBitmap.unwrap())() }
    }

    pub fn copy_frame_buffer_bitmap(&self) -> *mut LCDBitmap {
        unsafe { ((*self.handle).copyFrameBufferBitmap.unwrap())() }
    }

    pub fn mark_updated_rows(&self, start: i32, end: i32) {
        unsafe {
            ((*self.handle).markUpdatedRows.unwrap())(start, end);
        }
    }

    pub fn display(&self) {
        unsafe {
            ((*self.handle).display.unwrap())();
        }
    }

    pub fn set_color_to_pattern(
        &self,
        color: *mut LCDColor,
        bitmap: *mut LCDBitmap,
        x: i32,
        y: i32,
    ) {
        unsafe {
            ((*self.handle).setColorToPattern.unwrap())(color, bitmap, x, y);
        }
    }

    pub fn check_mask_collision(
        &self,
        bitmap1: *mut LCDBitmap,
        x1: i32,
        y1: i32,
        flip1: LCDBitmapFlip,
        bitmap2: *mut LCDBitmap,
        x2: i32,
        y2: i32,
        flip2: LCDBitmapFlip,
        rect: LCDRect,
    ) -> i32 {
        unsafe {
            ((*self.handle).checkMaskCollision.unwrap())(
                bitmap1, x1, y1, flip1, bitmap2, x2, y2, flip2, rect,
            )
        }
    }

    pub fn set_screen_clip_rect(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            ((*self.handle).setScreenClipRect.unwrap())(x, y, width, height);
        }
    }

    pub fn fill_polygon(
        &self,
        n_points: i32,
        coords: *mut i32,
        color: LCDColor,
        fillrule: LCDPolygonFillRule,
    ) {
        unsafe {
            ((*self.handle).fillPolygon.unwrap())(n_points, coords, color, fillrule);
        }
    }

    pub fn get_font_height(&self, font: *mut LCDFont) -> u8 {
        unsafe { ((*self.handle).getFontHeight.unwrap())(font) }
    }

    pub fn get_display_buffer_bitmap(&self) -> *mut LCDBitmap {
        unsafe { ((*self.handle).getDisplayBufferBitmap.unwrap())() }
    }

    pub fn draw_rotated_bitmap(
        &self,
        bitmap: *mut LCDBitmap,
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
                bitmap, x, y, rotation, centerx, centery, xscale, yscale,
            );
        }
    }

    pub fn set_text_leading(&self, line_height_adustment: i32) {
        unsafe {
            ((*self.handle).setTextLeading.unwrap())(line_height_adustment);
        }
    }

    pub fn set_bitmap_mask(&self, bitmap: *mut LCDBitmap, mask: *mut LCDBitmap) -> i32 {
        unsafe { ((*self.handle).setBitmapMask.unwrap())(bitmap, mask) }
    }

    pub fn get_bitmap_mask(&self, bitmap: *mut LCDBitmap) -> *mut LCDBitmap {
        unsafe { ((*self.handle).getBitmapMask.unwrap())(bitmap) }
    }

    pub fn set_stencil_image(&self, stencil: *mut LCDBitmap, tile: i32) {
        unsafe {
            ((*self.handle).setStencilImage.unwrap())(stencil, tile);
        }
    }

    pub fn make_font_from_data(&self, data: *mut LCDFontData, wide: i32) -> *mut LCDFont {
        unsafe { ((*self.handle).makeFontFromData.unwrap())(data, wide) }
    }
}
