#![no_std]

use playdate_rs::graphics::{Bitmap, LCDSolidColor};
use playdate_rs::{app, println, App, PLAYDATE};

#[app]
pub struct HelloWorld {
    image: Bitmap,
    rotation: f32,
}

unsafe impl Send for HelloWorld {}
unsafe impl Sync for HelloWorld {}

impl App for HelloWorld {
    fn new() -> Self {
        println!("Hello, World!");
        Self {
            image: PLAYDATE.graphics.load_bitmap("rust").unwrap(),
            rotation: 0f32,
        }
    }

    fn update(&mut self, delta: f32) {
        // Clear screen
        PLAYDATE.graphics.clear(LCDSolidColor::kColorWhite as _);
        // Draw image
        PLAYDATE.graphics.draw_rotated_bitmap(
            &self.image,
            130,
            120,
            self.rotation,
            0.5,
            0.5,
            1.0,
            1.0,
        );
        // Rotate image
        self.rotation += delta * 90.0;
        // Draw text
        PLAYDATE.graphics.draw_text("Hello, World!", 230, 112);
        // Draw FPS
        PLAYDATE.system.draw_fps(0, 0);
    }
}