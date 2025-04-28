#![no_std]

#[macro_use]
extern crate playdate_rs;
#[macro_use]
extern crate alloc;

use playdate_rs::graphics::{Color, Font};
use playdate_rs::network::tcp;
use playdate_rs::network::ws::WebSocket;
use playdate_rs::system::Buttons;
use playdate_rs::util::icons::FontIcons;
use playdate_rs::PLAYDATE;

#[main]
async fn main() {
    println!("Hello, WebSocket!");

    let _reply = tcp::request_access(None, 443, true, "I need access to the internet").await;

    println!("Access granted: {:?}", _reply);

    let font = Font::load("/System/Fonts/Roobert-10-Bold.pft").unwrap();
    PLAYDATE.graphics.set_font(&font);

    PLAYDATE.graphics.clear(Color::White);
    PLAYDATE.graphics.draw_text("Loading ...", vec2![50, 50]);

    let mut ws = WebSocket::connect("https://echo.websocket.org")
        .await
        .unwrap();

    let mut count = 0;
    let mut text = format!(
        "Press {} to send WebSocket message\n\nRESPONSE: N/A",
        FontIcons::A
    );

    loop {
        let _delta = PLAYDATE.next_frame().await;

        let pushed_buttons = PLAYDATE.system.get_button_state().pushed;
        if pushed_buttons.contains(Buttons::A) {
            count += 1;
            let msg = format!("Hello, WebSocket! {}", count);
            let t = PLAYDATE.system.get_current_time_milliseconds();
            ws.send(msg.as_bytes()).await.unwrap();
            let send_time = PLAYDATE.system.get_current_time_milliseconds() - t;
            println!("Sent: {}", msg);
            let t = PLAYDATE.system.get_current_time_milliseconds();
            let res = ws.recv_string().await.unwrap();
            let recv_time = PLAYDATE.system.get_current_time_milliseconds() - t;
            println!("Received: {}", res);
            text = format!(
                "Press {} to send WebSocket message\n\nRESPONSE: {}\n\nSend Latency: {}ms\nReceive Latency: {}ms",
                FontIcons::A,
                res,
                send_time,
                recv_time
            );
        }

        PLAYDATE.graphics.clear(Color::White);
        PLAYDATE.system.draw_fps(vec2![0, 0]);

        PLAYDATE.graphics.draw_text(&text, vec2![50, 50]);
    }
}
