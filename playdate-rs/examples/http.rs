#![no_std]

#[macro_use]
extern crate playdate_rs;
#[macro_use]
extern crate alloc;

use core::cell::RefCell;

use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use playdate_rs::graphics::{Color, Font};
use playdate_rs::graphics::{TextAlign, TextWrap};
use playdate_rs::network::http;
use playdate_rs::system::Buttons;
use playdate_rs::util::icons::FontIcons;
use playdate_rs::PLAYDATE;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
struct Coffee {
    id: u32,
    title: String,
    price: f32,
    description: String,
    image: String,
    ingredients: Vec<String>,
    total_sales: f32,
}

#[main]
async fn main() {
    println!("Hello, HTTP!");

    let _reply = http::request_access(None, 443, true, "I need access to the internet").await;

    let output: Arc<RefCell<Vec<Coffee>>> = Default::default();
    let output_clone = output.clone();

    spawn! { move:
        let response = http::get("https://api.sampleapis.com/coffee/hot", None).await;
        match response {
            Ok(resp) => {
                println!("Status Code: {:?}", resp.status_code);
                println!("Headers: {:?}", resp.headers);
                let coffee_list = resp.json::<Vec<Coffee>>().unwrap();
                *output_clone.borrow_mut() = coffee_list;
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    };

    let font = Font::load("/System/Fonts/Roobert-10-Bold.pft").unwrap();
    PLAYDATE.graphics.set_font(&font);

    let mut index = 0;

    loop {
        let _delta = PLAYDATE.next_frame().await;

        PLAYDATE.graphics.clear(Color::White);
        PLAYDATE.system.draw_fps(vec2![0, 0]);

        let coffee_list = output.borrow();
        if coffee_list.is_empty() {
            PLAYDATE
                .graphics
                .draw_text("Loading beverages list...", vec2![50, 50]);
            continue;
        }

        // Update index when button is pressed
        let pushed_buttons = PLAYDATE.system.get_button_state().pushed;
        if pushed_buttons.contains(Buttons::Left) {
            index = (index + coffee_list.len() - 1) % coffee_list.len();
        } else if pushed_buttons.contains(Buttons::Right) {
            index = (index + 1) % coffee_list.len();
        }

        let coffee = &coffee_list[index];

        let text = format!(
            "Beverages #{} in the list:\n\nName: {}\nPrice: ${:.2}\n\n{}",
            index, coffee.title, coffee.price, coffee.description
        );
        PLAYDATE.graphics.draw_text_in_rect(
            &text,
            rect!(x: 50, y: 50, w: 300, h: 200),
            TextWrap::Word,
            TextAlign::Left,
        );

        PLAYDATE.graphics.draw_text(
            &format!(
                "[Press {}/{} to nevigate]",
                FontIcons::LEFT,
                FontIcons::RIGHT
            ),
            vec2![50, 220],
        );
    }
}
