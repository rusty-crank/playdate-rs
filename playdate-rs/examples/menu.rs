#![no_std]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate playdate_rs;

use alloc::string::String;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicUsize, Ordering};

use playdate_rs::graphics::Color;
use playdate_rs::{main, println, PLAYDATE};

#[main]
async fn main() {
    println!("Hello, World!");

    let counter = Arc::new(AtomicUsize::new(0));

    let item1 = PLAYDATE.system.add_checkmark_menu_item("Rice", false);
    let item2 = PLAYDATE
        .system
        .add_options_menu_item("Fruit?", &["Apple", "Banana", "Orange"]);
    let mut item3 = PLAYDATE.system.add_menu_item("Counter");
    let counter_clone = counter.clone();
    item3.set_handler(move || {
        counter_clone.fetch_add(1, Ordering::SeqCst);
        async {}
    });

    loop {
        let _delta = PLAYDATE.next_frame().await;

        // Clear screen
        PLAYDATE.graphics.clear(Color::White);

        // Draw text
        let rice = item1.get_value() == 1;
        let fruit = match item2.get_value() {
            0 => "apple",
            1 => "banana",
            _ => "orange",
        };
        let count = counter.load(Ordering::SeqCst);

        let mut text = String::new();
        if rice {
            text.push_str("You ate some rice");
            match count {
                0 => text.push_str("!"),
                1 => text.push_str(&format!(" and 1 {}!", fruit)),
                _ => text.push_str(&format!(" and {} {}s!", count, fruit)),
            }
        } else if count > 0 {
            text.push_str(&format!("You ate {}", count));
            match count {
                1 => text.push_str(&format!(" {}!", fruit)),
                _ => text.push_str(&format!(" {}s!", fruit)),
            }
        } else {
            text.push_str("You didn't eat anything");
        }

        PLAYDATE.graphics.draw_text(text, vec2![50, 112]);

        // Draw FPS
        PLAYDATE.system.draw_fps(vec2![0, 0]);
    }
}
