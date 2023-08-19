use alloc::ffi::CString;

pub use sys::{LCDColor, LCDSolidColor, LCD_COLUMNS, LCD_ROWS, LCD_ROWSIZE};

pub struct Graphics {
    handle: *const sys::playdate_graphics,
}

impl Graphics {
    pub(crate) fn new(handle: *const sys::playdate_graphics) -> Self {
        Graphics { handle }
    }

    // pub video: *const playdate_video,

    pub fn clear(&self, color: sys::LCDColor) {
        unsafe {
            ((*self.handle).clear.unwrap())(color);
        }
    }

    // pub setBackgroundColor: ::core::option::Option<unsafe extern "C" fn(color: LCDSolidColor)>,
    // pub setStencil: ::core::option::Option<unsafe extern "C" fn(stencil: *mut LCDBitmap)>,
    // pub setDrawMode: ::core::option::Option<unsafe extern "C" fn(mode: LCDBitmapDrawMode)>,
    // pub setDrawOffset: ::core::option::Option<
    //     unsafe extern "C" fn(dx: ::core::ffi::c_int, dy: ::core::ffi::c_int),
    // >,
    // pub setClipRect: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         x: ::core::ffi::c_int,
    //         y: ::core::ffi::c_int,
    //         width: ::core::ffi::c_int,
    //         height: ::core::ffi::c_int,
    //     ),
    // >,
    // pub clearClipRect: ::core::option::Option<unsafe extern "C" fn()>,
    // pub setLineCapStyle: ::core::option::Option<unsafe extern "C" fn(endCapStyle: LCDLineCapStyle)>,
    // pub setFont: ::core::option::Option<unsafe extern "C" fn(font: *mut LCDFont)>,
    // pub setTextTracking: ::core::option::Option<unsafe extern "C" fn(tracking: ::core::ffi::c_int)>,
    // pub pushContext: ::core::option::Option<unsafe extern "C" fn(target: *mut LCDBitmap)>,
    // pub popContext: ::core::option::Option<unsafe extern "C" fn()>,
    // pub drawBitmap: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         bitmap: *mut LCDBitmap,
    //         x: ::core::ffi::c_int,
    //         y: ::core::ffi::c_int,
    //         flip: LCDBitmapFlip,
    //     ),
    // >,
    // pub tileBitmap: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         bitmap: *mut LCDBitmap,
    //         x: ::core::ffi::c_int,
    //         y: ::core::ffi::c_int,
    //         width: ::core::ffi::c_int,
    //         height: ::core::ffi::c_int,
    //         flip: LCDBitmapFlip,
    //     ),
    // >,
    // pub drawLine: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         x1: ::core::ffi::c_int,
    //         y1: ::core::ffi::c_int,
    //         x2: ::core::ffi::c_int,
    //         y2: ::core::ffi::c_int,
    //         width: ::core::ffi::c_int,
    //         color: LCDColor,
    //     ),
    // >,
    // pub fillTriangle: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         x1: ::core::ffi::c_int,
    //         y1: ::core::ffi::c_int,
    //         x2: ::core::ffi::c_int,
    //         y2: ::core::ffi::c_int,
    //         x3: ::core::ffi::c_int,
    //         y3: ::core::ffi::c_int,
    //         color: LCDColor,
    //     ),
    // >,
    // pub drawRect: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         x: ::core::ffi::c_int,
    //         y: ::core::ffi::c_int,
    //         width: ::core::ffi::c_int,
    //         height: ::core::ffi::c_int,
    //         color: LCDColor,
    //     ),
    // >,
    // pub fillRect: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         x: ::core::ffi::c_int,
    //         y: ::core::ffi::c_int,
    //         width: ::core::ffi::c_int,
    //         height: ::core::ffi::c_int,
    //         color: LCDColor,
    //     ),
    // >,
    // pub drawEllipse: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         x: ::core::ffi::c_int,
    //         y: ::core::ffi::c_int,
    //         width: ::core::ffi::c_int,
    //         height: ::core::ffi::c_int,
    //         lineWidth: ::core::ffi::c_int,
    //         startAngle: f32,
    //         endAngle: f32,
    //         color: LCDColor,
    //     ),
    // >,
    // pub fillEllipse: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         x: ::core::ffi::c_int,
    //         y: ::core::ffi::c_int,
    //         width: ::core::ffi::c_int,
    //         height: ::core::ffi::c_int,
    //         startAngle: f32,
    //         endAngle: f32,
    //         color: LCDColor,
    //     ),
    // >,
    // pub drawScaledBitmap: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         bitmap: *mut LCDBitmap,
    //         x: ::core::ffi::c_int,
    //         y: ::core::ffi::c_int,
    //         xscale: f32,
    //         yscale: f32,
    //     ),
    // >,

    pub fn draw_text(&self, text: impl AsRef<str>, x: i32, y: i32) -> i32 {
        unsafe {
            let c_string = CString::new(text.as_ref()).unwrap();
            ((*self.handle).drawText.unwrap())(
                c_string.as_ptr() as _,
                c_string.as_bytes().len(),
                sys::PDStringEncoding::kASCIIEncoding,
                x,
                y,
            )
        }
    }
    // pub newBitmap: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         width: ::core::ffi::c_int,
    //         height: ::core::ffi::c_int,
    //         bgcolor: LCDColor,
    //     ) -> *mut LCDBitmap,
    // >,
    // pub freeBitmap: ::core::option::Option<unsafe extern "C" fn(arg1: *mut LCDBitmap)>,
    // pub loadBitmap: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         path: *const ::core::ffi::c_char,
    //         outerr: *mut *const ::core::ffi::c_char,
    //     ) -> *mut LCDBitmap,
    // >,
    // pub copyBitmap:
    //     ::core::option::Option<unsafe extern "C" fn(bitmap: *mut LCDBitmap) -> *mut LCDBitmap>,
    // pub loadIntoBitmap: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         path: *const ::core::ffi::c_char,
    //         bitmap: *mut LCDBitmap,
    //         outerr: *mut *const ::core::ffi::c_char,
    //     ),
    // >,
    // pub getBitmapData: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         bitmap: *mut LCDBitmap,
    //         width: *mut ::core::ffi::c_int,
    //         height: *mut ::core::ffi::c_int,
    //         rowbytes: *mut ::core::ffi::c_int,
    //         mask: *mut *mut u8,
    //         data: *mut *mut u8,
    //     ),
    // >,
    // pub clearBitmap:
    //     ::core::option::Option<unsafe extern "C" fn(bitmap: *mut LCDBitmap, bgcolor: LCDColor)>,
    // pub rotatedBitmap: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         bitmap: *mut LCDBitmap,
    //         rotation: f32,
    //         xscale: f32,
    //         yscale: f32,
    //         allocedSize: *mut ::core::ffi::c_int,
    //     ) -> *mut LCDBitmap,
    // >,
    // pub newBitmapTable: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         count: ::core::ffi::c_int,
    //         width: ::core::ffi::c_int,
    //         height: ::core::ffi::c_int,
    //     ) -> *mut LCDBitmapTable,
    // >,
    // pub freeBitmapTable: ::core::option::Option<unsafe extern "C" fn(table: *mut LCDBitmapTable)>,
    // pub loadBitmapTable: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         path: *const ::core::ffi::c_char,
    //         outerr: *mut *const ::core::ffi::c_char,
    //     ) -> *mut LCDBitmapTable,
    // >,
    // pub loadIntoBitmapTable: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         path: *const ::core::ffi::c_char,
    //         table: *mut LCDBitmapTable,
    //         outerr: *mut *const ::core::ffi::c_char,
    //     ),
    // >,
    // pub getTableBitmap: ::core::option::Option<
    //     unsafe extern "C" fn(table: *mut LCDBitmapTable, idx: ::core::ffi::c_int) -> *mut LCDBitmap,
    // >,
    // pub loadFont: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         path: *const ::core::ffi::c_char,
    //         outErr: *mut *const ::core::ffi::c_char,
    //     ) -> *mut LCDFont,
    // >,
    // pub getFontPage: ::core::option::Option<
    //     unsafe extern "C" fn(font: *mut LCDFont, c: u32) -> *mut LCDFontPage,
    // >,
    // pub getPageGlyph: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         page: *mut LCDFontPage,
    //         c: u32,
    //         bitmap: *mut *mut LCDBitmap,
    //         advance: *mut ::core::ffi::c_int,
    //     ) -> *mut LCDFontGlyph,
    // >,
    // pub getGlyphKerning: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         glyph: *mut LCDFontGlyph,
    //         glyphcode: u32,
    //         nextcode: u32,
    //     ) -> ::core::ffi::c_int,
    // >,
    // pub getTextWidth: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         font: *mut LCDFont,
    //         text: *const ::core::ffi::c_void,
    //         len: usize,
    //         encoding: PDStringEncoding,
    //         tracking: ::core::ffi::c_int,
    //     ) -> ::core::ffi::c_int,
    // >,
    // pub getFrame: ::core::option::Option<unsafe extern "C" fn() -> *mut u8>,
    // pub getDisplayFrame: ::core::option::Option<unsafe extern "C" fn() -> *mut u8>,
    // pub getDebugBitmap: ::core::option::Option<unsafe extern "C" fn() -> *mut LCDBitmap>,
    // pub copyFrameBufferBitmap: ::core::option::Option<unsafe extern "C" fn() -> *mut LCDBitmap>,
    // pub markUpdatedRows: ::core::option::Option<
    //     unsafe extern "C" fn(start: ::core::ffi::c_int, end: ::core::ffi::c_int),
    // >,
    // pub display: ::core::option::Option<unsafe extern "C" fn()>,
    // pub setColorToPattern: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         color: *mut LCDColor,
    //         bitmap: *mut LCDBitmap,
    //         x: ::core::ffi::c_int,
    //         y: ::core::ffi::c_int,
    //     ),
    // >,
    // pub checkMaskCollision: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         bitmap1: *mut LCDBitmap,
    //         x1: ::core::ffi::c_int,
    //         y1: ::core::ffi::c_int,
    //         flip1: LCDBitmapFlip,
    //         bitmap2: *mut LCDBitmap,
    //         x2: ::core::ffi::c_int,
    //         y2: ::core::ffi::c_int,
    //         flip2: LCDBitmapFlip,
    //         rect: LCDRect,
    //     ) -> ::core::ffi::c_int,
    // >,
    // pub setScreenClipRect: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         x: ::core::ffi::c_int,
    //         y: ::core::ffi::c_int,
    //         width: ::core::ffi::c_int,
    //         height: ::core::ffi::c_int,
    //     ),
    // >,
    // pub fillPolygon: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         nPoints: ::core::ffi::c_int,
    //         coords: *mut ::core::ffi::c_int,
    //         color: LCDColor,
    //         fillrule: LCDPolygonFillRule,
    //     ),
    // >,
    // pub getFontHeight: ::core::option::Option<unsafe extern "C" fn(font: *mut LCDFont) -> u8>,
    // pub getDisplayBufferBitmap: ::core::option::Option<unsafe extern "C" fn() -> *mut LCDBitmap>,
    // pub drawRotatedBitmap: ::core::option::Option<
    //     unsafe extern "C" fn(
    //         bitmap: *mut LCDBitmap,
    //         x: ::core::ffi::c_int,
    //         y: ::core::ffi::c_int,
    //         rotation: f32,
    //         centerx: f32,
    //         centery: f32,
    //         xscale: f32,
    //         yscale: f32,
    //     ),
    // >,
    // pub setTextLeading:
    //     ::core::option::Option<unsafe extern "C" fn(lineHeightAdustment: ::core::ffi::c_int)>,
    // pub setBitmapMask: ::core::option::Option<
    //     unsafe extern "C" fn(bitmap: *mut LCDBitmap, mask: *mut LCDBitmap) -> ::core::ffi::c_int,
    // >,
    // pub getBitmapMask:
    //     ::core::option::Option<unsafe extern "C" fn(bitmap: *mut LCDBitmap) -> *mut LCDBitmap>,
    // pub setStencilImage: ::core::option::Option<
    //     unsafe extern "C" fn(stencil: *mut LCDBitmap, tile: ::core::ffi::c_int),
    // >,
    // pub makeFontFromData: ::core::option::Option<
    //     unsafe extern "C" fn(data: *mut LCDFontData, wide: ::core::ffi::c_int) -> *mut LCDFont,
    // >,
}
