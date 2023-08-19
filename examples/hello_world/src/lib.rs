#![no_std]

use core::cell::RefCell;

use playdate_rs::graphics::{LCDSolidColor, LCD_COLUMNS, LCD_ROWS};
use playdate_rs::{println, App, PLAYDATE};

const TEXT_WIDTH: i32 = 86;
const TEXT_HEIGHT: i32 = 16;

struct TextLoc {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

pub struct HelloWorld {
    text_loc: RefCell<TextLoc>,
}

unsafe impl Send for HelloWorld {}
unsafe impl Sync for HelloWorld {}

#[playdate_rs::app]
static HELLO_WORLD: HelloWorld = HelloWorld {
    text_loc: RefCell::new(TextLoc {
        x: (400 - TEXT_WIDTH) / 2,
        y: (240 - TEXT_HEIGHT) / 2,
        dx: 1,
        dy: 2,
    }),
};

impl App for HelloWorld {
    fn init(&self) {
        println!("Hello, World!");
    }

    fn update(&self) {
        // Clear screen
        PLAYDATE.graphics.clear(LCDSolidColor::kColorWhite as _);
        // Draw text
        let mut loc = self.text_loc.borrow_mut();
        PLAYDATE.graphics.draw_text("Hello, World!", loc.x, loc.y);
        // Update text location
        loc.x += loc.dx;
        loc.y += loc.dy;
        if loc.x < 0 || loc.x > LCD_COLUMNS as i32 - TEXT_WIDTH {
            loc.dx = -loc.dx;
        }
        if loc.y < 0 || loc.y > LCD_ROWS as i32 - TEXT_HEIGHT {
            loc.dy = -loc.dy;
        }
        // Draw FPS
        PLAYDATE.system.draw_fps(0, 0);
    }
}
