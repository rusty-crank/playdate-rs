#![no_std]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate playdate_rs;

use alloc::sync::Arc;
use core::cell::RefCell;

use playdate_rs::graphics::Color;
use playdate_rs::PLAYDATE;

#[main]
async fn main() {
    println!("Hello, World!");

    let counter = Arc::new(RefCell::new(0));

    let counter_clone = counter.clone();

    spawn! {
        loop {
            PLAYDATE.sleep(1000).await;
            *counter_clone.borrow_mut() += 1;
        }
    }

    loop {
        let _delta = PLAYDATE.next_frame().await;

        PLAYDATE.graphics.clear(Color::White);
        let count = *counter.borrow();
        PLAYDATE
            .graphics
            .draw_text(&format!("COUNTER: {}", count), vec2![70, 112]);
    }
}
