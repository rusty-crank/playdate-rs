#![no_std]

use core::sync::atomic::{AtomicI32, Ordering};

use playdate_rs::graphics::{LCDSolidColor, LCD_COLUMNS, LCD_ROWS};
use playdate_rs::{register_playdate_app, App, PLAYDATE};

const TEXT_WIDTH: i32 = 86;
const TEXT_HEIGHT: i32 = 16;

pub struct HelloWorld {
    x: AtomicI32,
    y: AtomicI32,
    dx: AtomicI32,
    dy: AtomicI32,
}

static HELLO_WORLD: HelloWorld = HelloWorld {
    x: AtomicI32::new((400 - TEXT_WIDTH) / 2),
    y: AtomicI32::new((240 - TEXT_HEIGHT) / 2),
    dx: AtomicI32::new(1),
    dy: AtomicI32::new(2),
};

impl App for HelloWorld {
    fn update(&self) {
        PLAYDATE.graphics.clear(LCDSolidColor::kColorWhite as _);
        PLAYDATE.graphics.draw_text(
            "Hello, World!",
            self.x.load(Ordering::Relaxed),
            self.y.load(Ordering::Relaxed),
        );
        self.x
            .fetch_add(self.dx.load(Ordering::Relaxed), Ordering::Relaxed);
        self.y
            .fetch_add(self.dy.load(Ordering::Relaxed), Ordering::Relaxed);
        if self.x.load(Ordering::Relaxed) < 0
            || self.x.load(Ordering::Relaxed) > LCD_COLUMNS as i32 - TEXT_WIDTH
        {
            self.dx
                .store(-self.dx.load(Ordering::Relaxed), Ordering::Relaxed);
        }
        if self.y.load(Ordering::Relaxed) < 0
            || self.y.load(Ordering::Relaxed) > LCD_ROWS as i32 - TEXT_HEIGHT
        {
            self.dy
                .store(-self.dy.load(Ordering::Relaxed), Ordering::Relaxed);
        }
        PLAYDATE.system.draw_fps(0, 0);
    }
}

register_playdate_app!(HELLO_WORLD);
