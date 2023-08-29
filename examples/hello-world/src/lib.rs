#![no_std]

use playdate_rs::graphics::{Bitmap, LCDSolidColor};
use playdate_rs::math::euclid::default::Vector2D;
use playdate_rs::math::Point2D;
use playdate_rs::{app, println, App, PLAYDATE};

#[app]
pub struct HelloWorld {
    image: Bitmap,
    rotation: f32,
}

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
        PLAYDATE.graphics.clear(LCDSolidColor::kColorWhite);
        // Draw image
        PLAYDATE.graphics.draw_rotated_bitmap(
            &self.image,
            Point2D::new(130, 120),
            self.rotation,
            Point2D::new(0.5, 0.5),
            Vector2D::new(1.0, 1.0),
        );
        // Rotate image
        self.rotation += delta * 90.0;
        // Draw text
        PLAYDATE
            .graphics
            .draw_text("Hello, World!", Point2D::new(230, 112));
        // Draw FPS
        PLAYDATE.system.draw_fps(Point2D::new(0, 0));
    }
}
