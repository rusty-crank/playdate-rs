use core::{ffi::c_char, marker::PhantomData};

use crate::{
    fs::AsPath,
    math::{Size, Vec2},
};
use alloc::ffi::CString;

use crate::{math::SideOffsets, util::Ref};

use sys::{
    LCDBitmapFlip as BitmapFlip, LCDColor as ColorOrPattern, LCDPattern as Pattern,
    LCDSolidColor as Color,
};

use crate::{error::Error, PLAYDATE};

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
    pub fn new(size: Size<u32>, bgcolor: impl Into<ColorOrPattern>) -> Self {
        Self::from(unsafe {
            ((*PLAYDATE.graphics.handle).newBitmap.unwrap())(
                size.width as _,
                size.height as _,
                bgcolor.into(),
            )
        })
    }

    /// Open an image as a bitmap.
    pub fn load(path: impl AsPath) -> Result<Self, Error> {
        unsafe {
            let c_string = CString::new(path.as_str().as_ref()).unwrap();
            let mut err: *const c_char = core::ptr::null();
            let ptr =
                ((*PLAYDATE.graphics.handle).loadBitmap.unwrap())(c_string.as_ptr() as _, &mut err);
            if !err.is_null() {
                let err = CString::from_raw(err as *mut c_char);
                let err = err.into_string().unwrap();
                return Err(Error::FailedToLoadBitMapFromFile(err));
            }
            Ok(Bitmap::from(ptr))
        }
    }

    /// Clears bitmap, filling with the given bgcolor.
    pub fn clear(&self, bgcolor: impl Into<ColorOrPattern>) {
        unsafe { ((*PLAYDATE.graphics.handle).clearBitmap.unwrap())(self.handle, bgcolor.into()) }
    }

    /// Sets a mask image for the given bitmap. The set mask must be the same size as the target bitmap.
    pub fn set_mask(&self, mask: impl AsRef<Bitmap>) -> Result<(), Error> {
        let result = unsafe {
            ((*PLAYDATE.graphics.handle).setBitmapMask.unwrap())(self.handle, mask.as_ref().handle)
        };
        if result != 1 {
            Err(Error::FailedToSetBitmapMask)
        } else {
            Ok(())
        }
    }

    /// Gets a mask image for the given bitmap. If the image doesn’t have a mask, getBitmapMask returns NULL.
    pub fn get_mask(&self) -> Ref<Bitmap> {
        Self::from_ref(unsafe { ((*PLAYDATE.graphics.handle).getBitmapMask.unwrap())(self.handle) })
    }

    /// Returns `true` if any of the opaque pixels in `self` when positioned at `x1`, `y1` with `flip1` overlap any of the opaque pixels in `other` at `x2`, `y2` with `flip2` within the non-empty rect, or `false` if no pixels overlap or if one or both fall completely outside of rect.
    #[allow(clippy::too_many_arguments)]
    pub fn check_mask_collision(
        &self,
        x1: i32,
        y1: i32,
        flip1: BitmapFlip,
        other: impl AsRef<Bitmap>,
        x2: i32,
        y2: i32,
        flip2: BitmapFlip,
        rect: SideOffsets<i32>,
    ) -> bool {
        unsafe {
            ((*PLAYDATE.graphics.handle).checkMaskCollision.unwrap())(
                self.handle,
                x1,
                y1,
                flip1,
                other.as_ref().handle,
                x2,
                y2,
                flip2,
                rect.into(),
            ) == 1
        }
    }

    /// Gets various info about bitmap including its width and height and raw pixel data. The data is 1 bit per pixel packed format, in MSB order; in other words, the high bit of the first byte in data is the top left pixel of the image. If the bitmap has a mask, a pointer to its data is returned in mask, else NULL is returned.
    pub fn get_bitmap_data(&self) -> BitmapData {
        let mut data = BitmapData::new();
        unsafe {
            ((*PLAYDATE.graphics.handle).getBitmapData.unwrap())(
                self.handle,
                &mut data.size.width,
                &mut data.size.height,
                &mut data.rowbytes,
                &mut data.mask,
                &mut data.data,
            )
        }
        data
    }

    /// Gets the color of the pixel at (x,y) in the given bitmap. If the coordinate is outside the bounds of the bitmap, or if the bitmap has a mask and the pixel is marked transparent, the function returns kColorClear; otherwise the return value is kColorWhite or kColorBlack.
    pub fn get_pixel(&self, pos: Vec2<i32>) -> Color {
        unsafe { ((*PLAYDATE.graphics.handle).getBitmapPixel.unwrap())(self.handle, pos.x, pos.y) }
    }

    /// Loads the image at path into the previously allocated bitmap.
    pub fn load_from_file(&self, path: impl AsPath) -> Result<(), Error> {
        let c_string = CString::new(path.as_str().as_ref()).unwrap();
        let mut err: *const c_char = core::ptr::null();
        unsafe {
            ((*PLAYDATE.graphics.handle).loadIntoBitmap.unwrap())(
                c_string.as_ptr() as _,
                self.handle,
                &mut err,
            )
        }
        if !err.is_null() {
            let err = unsafe { CString::from_raw(err as *mut c_char) };
            let err = err.into_string().unwrap();
            return Err(Error::FailedToLoadBitMapFromFile(err));
        }
        Ok(())
    }

    /// Returns a new, rotated and scaled LCDBitmap based on the given bitmap.
    pub fn rotated(&self, rotation: f32, scale: Vec2<f32>) -> Bitmap {
        let mut alloced_size = 0;
        Self::from(unsafe {
            ((*PLAYDATE.graphics.handle).rotatedBitmap.unwrap())(
                self.handle,
                rotation,
                scale.x,
                scale.y,
                &mut alloced_size,
            )
        })
    }

    /// Get color as an 8 x 8 pattern using the given bitmap. x, y indicates the top left corner of the 8 x 8 pattern.
    pub fn get_color_pattern(&self, pos: Vec2<i32>) -> ColorPatternData {
        let mut color = ColorOrPattern::default();
        unsafe {
            ((*PLAYDATE.graphics.handle).setColorToPattern.unwrap())(
                &mut color,
                self.handle,
                pos.x,
                pos.y,
            );
        }
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
        unsafe { ((*PLAYDATE.graphics.handle).freeBitmap.unwrap())(self.handle) }
    }
}

impl Clone for Bitmap {
    fn clone(&self) -> Self {
        Bitmap {
            handle: unsafe { ((*PLAYDATE.graphics.handle).copyBitmap.unwrap())(self.handle) },
        }
    }
}

pub enum ColorPatternData {
    Solid(Color),
    Pattern(Pattern),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BitmapData<'a> {
    pub size: Size<i32>,
    pub rowbytes: i32,
    pub mask: *mut u8,
    pub data: *mut u8,
    _p: PhantomData<&'a ()>,
}

impl<'a> BitmapData<'a> {
    fn new() -> Self {
        BitmapData {
            size: Size::default(),
            rowbytes: 0,
            mask: core::ptr::null_mut(),
            data: core::ptr::null_mut(),
            _p: PhantomData,
        }
    }

    /// Get the value of a pixel at x, y. Returns true if the pixel is black
    pub fn get_pixel(&self, pos: Vec2<u32>) -> bool {
        let byte_index = pos.y * self.rowbytes as u32 + pos.x / 8;
        let byte_ptr = unsafe { self.data.add(byte_index as _) };
        let v = unsafe { *byte_ptr };
        let bit_index = pos.x % 8;
        let mask = 1 << (7 - bit_index);
        v & mask != 0
    }
}

/// There are two kinds of image tables: matrix and sequential.
///
/// Matrix image tables are great as sources of imagery for tilemap. They are loaded from a single file in your game’s source folder with the suffix -table-<w>-<h> before the file extension. The compiler splits the image into separate bitmaps of dimension w by h pixels that are accessible via imagetable:getImage(x,y).
///
/// Sequential image tables are useful as a way to load up sequential frames of animation. They are loaded from a sequence of files in your game’s source folder at compile time from filenames with the suffix -table-<sequenceNumber> before the file extension. Individual images in the sequence are accessible via imagetable:getImage(n). The images employed by a sequential image table are not required to be the same size, unlike the images used in a matrix image table.
#[derive(PartialEq, Eq, Debug)]
pub struct BitmapTable {
    pub(crate) handle: *mut sys::LCDBitmapTable,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct BitmapTableInfo {
    pub count: usize,
    pub cellswide: usize,
}

unsafe impl Send for BitmapTable {}
unsafe impl Sync for BitmapTable {}

impl BitmapTable {
    fn from(handle: *mut sys::LCDBitmapTable) -> Self {
        Self { handle }
    }

    /// Allocates and returns a new LCDBitmapTable that can hold count width by height LCDBitmaps.
    pub fn new(count: usize, width: u32, height: u32) -> Self {
        BitmapTable::from(unsafe {
            ((*PLAYDATE.graphics.handle).newBitmapTable.unwrap())(
                count as _,
                width as _,
                height as _,
            )
        })
    }

    /// Allocates and returns a new LCDBitmap from the file at path. If there is no file at path, the function returns null.
    pub fn load(path: impl AsPath) -> Result<BitmapTable, Error> {
        unsafe {
            let c_string = CString::new(path.as_str().as_ref()).unwrap();
            let mut err = core::ptr::null();
            let ptr = ((*PLAYDATE.graphics.handle).loadBitmapTable.unwrap())(
                c_string.as_ptr() as _,
                &mut err,
            );
            if !err.is_null() {
                let err = CString::from_raw(err as *mut c_char);
                let err = err.into_string().unwrap();
                return Err(Error::FailedToLoadBitMapTableFromFile(err));
            }
            Ok(BitmapTable::from(ptr))
        }
    }

    pub fn get_info(&self) -> BitmapTableInfo {
        let mut count = 0;
        let mut cellswide = 0;
        unsafe {
            ((*PLAYDATE.graphics.handle).getBitmapTableInfo.unwrap())(
                self.handle,
                &mut count,
                &mut cellswide,
            )
        };
        BitmapTableInfo {
            count: count as _,
            cellswide: cellswide as _,
        }
    }

    /// Returns the idx bitmap in table, If idx is out of bounds, the function returns NULL.
    pub fn get(&self, idx: usize) -> Option<Ref<Bitmap>> {
        let ptr =
            unsafe { ((*PLAYDATE.graphics.handle).getTableBitmap.unwrap())(self.handle, idx as _) };
        if ptr.is_null() {
            return None;
        }
        Some(Bitmap::from_ref(ptr))
    }

    /// Allocates and returns a new LCDBitmap from the file at path. If there is no file at path, the function returns null.
    pub fn load_from_file(&mut self, path: impl AsPath) -> Result<(), Error> {
        let c_string = CString::new(path.as_str().as_ref()).unwrap();
        let mut err: *const c_char = core::ptr::null();
        unsafe {
            ((*PLAYDATE.graphics.handle).loadIntoBitmapTable.unwrap())(
                c_string.as_ptr() as _,
                self.handle,
                &mut err,
            )
        }
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
        unsafe { ((*PLAYDATE.graphics.handle).freeBitmapTable.unwrap())(self.handle) }
    }
}
