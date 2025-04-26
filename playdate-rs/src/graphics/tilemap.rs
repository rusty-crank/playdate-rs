use crate::math::{Size, Vec2};

use crate::PLAYDATE;

use super::BitmapTable;

#[derive(PartialEq, Eq, Debug)]
pub struct TileMap {
    handle: *mut sys::LCDTileMap,
    image_table: Option<BitmapTable>,
}

impl TileMap {
    pub fn new() -> Self {
        Self {
            handle: unsafe { ((*(*PLAYDATE.graphics.handle).tilemap).newTilemap.unwrap())() },
            image_table: None,
        }
    }

    /// Sets the image table to use for the tilemap’s tiles.
    pub fn set_image_table(&mut self, table: BitmapTable) {
        let handle = table.handle;
        self.image_table = Some(table);
        unsafe {
            ((*(*PLAYDATE.graphics.handle).tilemap)
                .setImageTable
                .unwrap())(self.handle, handle)
        }
    }

    /// Returns the LCDBitmapTable used for the tilemap’s tiles.
    pub fn get_image_table(&self) -> Option<&BitmapTable> {
        if let Some(ref table) = self.image_table {
            Some(table)
        } else {
            None
        }
    }

    /// Sets the tilemap’s width and height, in number of tiles.
    pub fn set_size(&mut self, tiles: Size<u32>) {
        unsafe {
            ((*(*PLAYDATE.graphics.handle).tilemap).setSize.unwrap())(
                self.handle,
                tiles.width as _,
                tiles.height as _,
            )
        }
    }

    /// Returns the size of the tile map, in tiles.
    pub fn get_size(&self) -> Size<u32> {
        let mut size = Size::default();
        unsafe {
            ((*(*PLAYDATE.graphics.handle).tilemap).getSize.unwrap())(
                self.handle,
                &mut size.width,
                &mut size.height,
            )
        }
        Size {
            width: size.width as _,
            height: size.height as _,
        }
    }

    /// Returns the size of the tilemap in pixels; that is, the size of the tile image multiplied by the number of rows and columns in the tilemap.
    pub fn get_pixel_size(&self) -> Size<u32> {
        let mut size = Size::default();
        unsafe {
            ((*(*PLAYDATE.graphics.handle).tilemap).getPixelSize.unwrap())(
                self.handle,
                &mut size.width,
                &mut size.height,
            )
        }
        size
    }

    /// Sets the tilemap’s width to rowwidth and height to count/rowwidth (count must be evenly divisible by rowwidth), then sets the tiles' indexes to the given list.
    pub fn set_tiles(&mut self, indexes: &[u16], row_width: i32) {
        unsafe {
            ((*(*PLAYDATE.graphics.handle).tilemap).setTiles.unwrap())(
                self.handle,
                indexes.as_ptr() as *const u16 as *mut u16,
                indexes.len() as _,
                row_width,
            )
        }
    }

    /// Sets the index of the tile at tilemap position (x, y). index is the (0-based) index of the cell in the tilemap’s image table.
    pub fn set_tile_at_position(&mut self, pos: Vec2<u32>, idx: usize) {
        unsafe {
            ((*(*PLAYDATE.graphics.handle).tilemap)
                .setTileAtPosition
                .unwrap())(self.handle, pos.x as _, pos.y as _, idx as _)
        }
    }

    /// Returns the image index of the tile at the given x and y coordinate. If x or y is out of bounds, returns -1.
    pub fn get_tile_at_position(&self, pos: Vec2<u32>) -> Option<usize> {
        let v = unsafe {
            ((*(*PLAYDATE.graphics.handle).tilemap)
                .getTileAtPosition
                .unwrap())(self.handle, pos.x as _, pos.y as _)
        };
        if v == -1 {
            None
        } else {
            Some(v as _)
        }
    }

    /// Draws the tile map at coordinate (x, y).
    pub fn draw_at_point(&self, pos: Vec2<f32>) {
        unsafe {
            ((*(*PLAYDATE.graphics.handle).tilemap).drawAtPoint.unwrap())(self.handle, pos.x, pos.y)
        }
    }
}

impl Drop for TileMap {
    fn drop(&mut self) {
        unsafe { ((*(*PLAYDATE.graphics.handle).tilemap).freeTilemap.unwrap())(self.handle) }
    }
}
