use crate::{fs::AsPath, util::Ref};
use alloc::ffi::CString;

use crate::PLAYDATE;

use super::Bitmap;
use crate::error::Error;

#[derive(PartialEq, Eq, Debug)]
pub struct Font {
    pub(crate) handle: *mut sys::LCDFont,
}

unsafe impl Send for Font {}
unsafe impl Sync for Font {}

impl Font {
    /// Returns the LCDFont object for the font file at path. In case of error, outErr points to a string describing the error.
    pub fn load(path: impl AsPath) -> Result<Font, Error> {
        unsafe {
            let c_string = CString::new(path.as_str().as_ref()).unwrap();
            let mut err = core::ptr::null();
            let font =
                ((*PLAYDATE.graphics.handle).loadFont.unwrap())(c_string.as_ptr() as _, &mut err);
            if !err.is_null() {
                let err = CString::from_raw(err as *mut core::ffi::c_char);
                let err = err.into_string().unwrap();
                return Err(Error::FailedToLoadFont(err));
            }
            Ok(Font { handle: font })
        }
    }

    /// Returns an LCDFont object wrapping the LCDFontData data comprising the contents (minus 16-byte header) of an uncompressed pft file. wide corresponds to the flag in the header indicating whether the font contains glyphs at codepoints above U+1FFFF.
    /// # Safety
    /// Assumes that the LCDFontData is valid.
    pub unsafe fn from_font_data(data: *mut sys::LCDFontData, wide: i32) -> Font {
        unsafe {
            let font = ((*PLAYDATE.graphics.handle).makeFontFromData.unwrap())(data, wide);
            Font { handle: font }
        }
    }

    /// Returns the height of the given font.
    pub fn get_height(&self) -> u8 {
        unsafe { ((*PLAYDATE.graphics.handle).getFontHeight.unwrap())(self.handle) }
    }

    /// Returns the width of the given text in the given font.
    pub fn get_text_width(&self, text: impl AsRef<str>, tracking: i32) -> u32 {
        let ptr = text.as_ref().as_ptr() as *const core::ffi::c_void;
        let len = text.as_ref().chars().count();
        unsafe {
            ((*PLAYDATE.graphics.handle).getTextWidth.unwrap())(
                self.handle,
                ptr,
                len,
                sys::PDStringEncoding::UTF8,
                tracking,
            ) as _
        }
    }

    /// Returns an LCDFontPage object for the given character code. Each LCDFontPage contains information for 256 characters; specifically, if (c1 & ~0xff) == (c2 & ~0xff), then c1 and c2 belong to the same page and the same LCDFontPage can be used to fetch the character data for both instead of searching for the page twice.
    pub fn get_page(&self, c: u32) -> FontPage {
        FontPage::new(unsafe { ((*PLAYDATE.graphics.handle).getFontPage.unwrap())(self.handle, c) })
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct FontPage {
    handle: *mut sys::LCDFontPage,
}

unsafe impl Send for FontPage {}
unsafe impl Sync for FontPage {}

impl FontPage {
    pub(crate) fn new(handle: *mut sys::LCDFontPage) -> Self {
        Self { handle }
    }

    /// Returns an LCDFontGlyph object for character c in LCDFontPage page, and optionally returns the glyph’s bitmap and advance value.
    pub fn get_glyph(&self, c: u32) -> (FontGlyph, Option<Ref<Bitmap>>, Option<i32>) {
        let mut bitmap = core::ptr::null_mut();
        let mut advance = 0;
        let glyph = FontGlyph::new(unsafe {
            (*PLAYDATE.graphics.handle).getPageGlyph.unwrap()(
                self.handle,
                c,
                &mut bitmap,
                &mut advance,
            )
        });
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
        unsafe { ((*PLAYDATE.graphics.handle).getGlyphKerning.unwrap())(self.handle, c1, c2) }
    }
}
