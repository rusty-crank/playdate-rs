#![no_std]

extern crate alloc;

use core::ops::{Add, Mul};

use alloc::format;
use playdate_rs::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use playdate_rs::graphics::LCDSolidColor;
use playdate_rs::math::Point2D;
use playdate_rs::system::Buttons;
use playdate_rs::{app, println, App, PLAYDATE};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex {
    re: f32,
    im: f32,
}

impl Complex {
    fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }

    fn from_point(point: Point2D<i32>, center: Complex, scale: f32) -> Self {
        let bounds = (DISPLAY_WIDTH as f32, DISPLAY_HEIGHT as f32);
        let c1 = Complex::new(
            center.re - bounds.0 / 2.0 * scale,
            center.im + bounds.1 / 2.0 * scale,
        );
        let c2 = Complex::new(
            center.re + bounds.0 / 2.0 * scale,
            center.im - bounds.1 / 2.0 * scale,
        );
        let upper_left = Complex::new(c1.re.min(c2.re), c1.im.max(c2.im));
        let lower_right = Complex::new(c1.re.max(c2.re), c1.im.min(c2.im));
        assert!(lower_right.re > upper_left.re);
        assert!(upper_left.im > lower_right.im);
        let (width, height) = (
            lower_right.re - upper_left.re,
            upper_left.im - lower_right.im,
        );
        Complex {
            re: upper_left.re + point.x as f32 * width / bounds.0,
            im: upper_left.im - point.y as f32 * height / bounds.1,
        }
    }

    fn norm_sqr(&self) -> f32 {
        self.re * self.re + self.im * self.im
    }
}

impl Add<Complex> for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl Mul<Complex> for Complex {
    type Output = Self;
    fn mul(self, rhs: Complex) -> Self::Output {
        Self::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

fn f(c: Complex, max_iter: i32) -> bool {
    let mut z = Complex::new(0.0, 0.0);
    for _ in 0..max_iter {
        if z.norm_sqr() > 4.0 {
            return true;
        }
        z = z * z + c;
    }
    false
}

#[app]
pub struct Mandelbrot {
    center: Complex,
    scale: f32,
}

impl Mandelbrot {
    fn get_iter(&self) -> i32 {
        match self.scale {
            s if s > 0.01 => 16,
            s if s > 0.001 => 32,
            s if s > 0.0001 => 64,
            s if s > 0.00001 => 96,
            _ => 128,
        }
    }

    fn draw_frame(&self) {
        PLAYDATE.graphics.clear(LCDSolidColor::kColorWhite);
        let iter = self.get_iter();
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let c = Point2D::new(x as i32, y as i32);
                let v = f(Complex::from_point(c, self.center, self.scale), iter);
                PLAYDATE.graphics.draw_pixel(
                    x as i32,
                    y as i32,
                    if v {
                        LCDSolidColor::kColorBlack
                    } else {
                        LCDSolidColor::kColorWhite
                    },
                );
            }
        }
    }

    fn draw_meta(&self) {
        let text_area_width = 130;
        let row_height = 14;
        let text_area_height = row_height * 3;
        let top_left_x = DISPLAY_WIDTH as i32 - text_area_width;
        PLAYDATE.graphics.draw_rect(
            top_left_x - 3,
            DISPLAY_HEIGHT as i32 - text_area_height - 3,
            text_area_width + 3,
            text_area_height + 3,
            LCDSolidColor::kColorWhite,
        );
        PLAYDATE.graphics.fill_rect(
            top_left_x - 2,
            DISPLAY_HEIGHT as i32 - text_area_height - 2,
            text_area_width + 2,
            text_area_height + 2,
            LCDSolidColor::kColorBlack,
        );
        PLAYDATE.graphics.fill_rect(
            top_left_x,
            DISPLAY_HEIGHT as i32 - text_area_height,
            text_area_width,
            text_area_height,
            LCDSolidColor::kColorWhite,
        );
        PLAYDATE.graphics.draw_text(
            format!("<{:.4}, {:.4}i>", self.center.re, self.center.im,),
            top_left_x + 2,
            DISPLAY_HEIGHT as i32 - row_height * 3,
        );
        PLAYDATE.graphics.draw_text(
            format!("SCALE: {:.8}", 1.0 / (self.scale * 100.0)),
            top_left_x + 2,
            DISPLAY_HEIGHT as i32 - row_height * 2,
        );
        PLAYDATE.graphics.draw_text(
            format!("ITER: {:}", self.get_iter()),
            top_left_x + 2,
            DISPLAY_HEIGHT as i32 - row_height,
        );
    }
}

impl App for Mandelbrot {
    fn new() -> Self {
        println!("Hello, Mandelbrot!");
        Self {
            center: Complex::new(-0.5, 0.0),
            scale: 0.01,
        }
    }

    fn init(&mut self) {
        let font = PLAYDATE
            .graphics
            .load_font("/System/Fonts/Roobert-10-Bold.pft")
            .unwrap();
        PLAYDATE.graphics.set_font(&font);
        self.draw_frame();
    }

    fn update(&mut self, _delta: f32) {
        let button_state = PLAYDATE.system.get_button_state();
        let prev_scale = self.scale;
        let prev_center = self.center;
        // Scale
        if button_state.current.contains(Buttons::A) {
            self.scale /= 1.1;
        } else if button_state.current.contains(Buttons::B) {
            self.scale *= 1.1
        }
        if !PLAYDATE.system.is_crank_docked() {
            let crank = PLAYDATE.system.get_crank_change();
            if crank > 0.0 {
                self.scale /= 1.05;
            } else if crank < 0.0 {
                self.scale *= 1.05;
            }
        }
        // Move
        if button_state.current.contains(Buttons::Up) {
            self.center.im += 10.0 * self.scale
        } else if button_state.current.contains(Buttons::Down) {
            self.center.im -= 10.0 * self.scale
        }
        if button_state.current.contains(Buttons::Left) {
            self.center.re -= 10.0 * self.scale
        } else if button_state.current.contains(Buttons::Right) {
            self.center.re += 10.0 * self.scale
        }
        if prev_center != self.center || prev_scale != self.scale {
            self.draw_frame();
        }
        // Draw metadata
        self.draw_meta();
        // Draw FPS
        PLAYDATE.system.draw_fps(0, 0);
    }
}
