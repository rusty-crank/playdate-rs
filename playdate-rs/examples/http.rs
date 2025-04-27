#![no_std]

#[macro_use]
extern crate playdate_rs;

use playdate_rs::graphics::{Bitmap, Color};
use playdate_rs::network::http;
use playdate_rs::{main, println, PLAYDATE};

#[main]
async fn main() {
    let image = Bitmap::open("rust").unwrap();
    let mut rotation = 0f32;

    println!("Hello, World!");

    let _reply = http::request_access(None, 443, true, "I need access to the internet").await;

    PLAYDATE.spawn(async {
        let response = http::get("https://api.sampleapis.com/coffee/hot", None).await;
        match response {
            Ok(resp) => {
                println!("Response: {:?}", resp.status_code);
                println!("Headers: {:?}", resp.headers);
                let data = resp.data();
                let data_to_string = core::str::from_utf8(data).unwrap_or("Invalid UTF-8");
                println!("Body: {:?}", data_to_string);
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    });

    loop {
        let delta = PLAYDATE.next_frame().await;
        // Clear screen
        PLAYDATE.graphics.clear(Color::White);
        // Draw image
        PLAYDATE.graphics.draw_rotated_bitmap(
            &image,
            vec2![130, 120],
            rotation,
            vec2![0.5, 0.5],
            vec2![1.0, 1.0],
        );
        // Rotate image
        rotation += delta * 90.0;
        // Draw text
        PLAYDATE
            .graphics
            .draw_text("Hello, World!", vec2![230, 112]);
        // Draw FPS
        PLAYDATE.system.draw_fps(vec2![0, 0]);
    }
}
