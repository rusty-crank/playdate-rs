#![no_std]

use core::cell::RefCell;

use playdate_rs::graphics::{Bitmap, BitmapData, LCDSolidColor, LCD_COLUMNS, LCD_ROWS};
use playdate_rs::sys::LCDBitmapFlip;
use playdate_rs::{println, App, PLAYDATE};

const TEXT_WIDTH: i32 = 90;
const TEXT_HEIGHT: i32 = 16;

struct TextLoc {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

pub struct HelloWorld {
    text_loc: RefCell<TextLoc>,
    image: RefCell<Option<Bitmap>>,
}

unsafe impl Send for HelloWorld {}
unsafe impl Sync for HelloWorld {}

#[playdate_rs::app]
static HELLO_WORLD: HelloWorld = HelloWorld {
    text_loc: RefCell::new(TextLoc {
        x: (LCD_COLUMNS as i32 - TEXT_WIDTH) / 2,
        y: (LCD_ROWS as i32 - TEXT_HEIGHT) / 2,
        dx: 1,
        dy: 2,
    }),
    image: RefCell::new(None),
};

impl App for HelloWorld {
    fn init(&self) {
        println!("Hello, World!");
        *self.image.borrow_mut() = Some(PLAYDATE.graphics.load_bitmap("snowflake").unwrap());
    }

    fn update(&self) {
        // Clear screen
        PLAYDATE.graphics.clear(LCDSolidColor::kColorWhite as _);
        // Draw image
        let mut loc = self.text_loc.borrow_mut();
        let bitmap = self.image.borrow();
        let bitmap = bitmap.as_ref().unwrap();
        PLAYDATE
            .graphics
            .draw_bitmap(bitmap, loc.x, loc.y, LCDBitmapFlip::kBitmapUnflipped);
        // Draw text
        let BitmapData {
            width: bitmap_width,
            height: bitmap_height,
            ..
        } = bitmap.get_bitmap_data();
        let (margin_left, padding_top) = (4, 2);
        PLAYDATE.graphics.draw_text(
            "Hello, World!",
            loc.x + margin_left + bitmap_width,
            loc.y + padding_top,
        );
        let total_width = bitmap_width + margin_left + TEXT_WIDTH;
        let total_height = bitmap_height.max(TEXT_HEIGHT);
        // Update text location
        loc.x += loc.dx;
        loc.y += loc.dy;
        if loc.x < 0 || loc.x > LCD_COLUMNS as i32 - total_width {
            loc.dx = -loc.dx;
        }
        if loc.y < 0 || loc.y > LCD_ROWS as i32 - total_height {
            loc.dy = -loc.dy;
        }
        // Draw FPS
        PLAYDATE.system.draw_fps(0, 0);
    }
}
