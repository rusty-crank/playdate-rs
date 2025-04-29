#![no_std]

#[macro_use]
extern crate playdate_rs;
#[macro_use]
extern crate alloc;

use core::cell::RefCell;

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use playdate_rs::error::Error;
use playdate_rs::graphics::{Color, Font};
use playdate_rs::graphics::{TextAlign, TextWrap};
use playdate_rs::io::ErrorKind;
use playdate_rs::network::http::{self, Headers};
use playdate_rs::network::http::{AsyncStream, HTTPOptions};
use playdate_rs::system::Buttons;
use playdate_rs::util::icons::FontIcons;
use playdate_rs::PLAYDATE;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Start,
    Loading,
    Loaded,
}

const PROMPT: &str = "Tell me three different coffee names";

const MODEL: &str = "gpt-4o-mini";
const ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";
const API_KEY: Option<&str> = option_env!("OPENAI_API_KEY");

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Delta {
    content: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    delta: Delta,
}

#[derive(Deserialize, Debug)]
struct Chunk {
    choices: Vec<Choice>,
}

#[derive(Serialize, Debug)]
struct Request {
    model: String,
    messages: Vec<Message>,
    stream: Option<bool>,
}

async fn chat_completion(prompt: &str) -> impl AsyncStream<Item = Option<Chunk>> {
    println!("Chat completion: {}", prompt);
    let body = Request {
        model: MODEL.to_owned(),
        messages: vec![Message {
            role: "user".to_owned(),
            content: Some(prompt.to_owned()),
        }],
        stream: Some(true),
    };
    let headers = Headers::new()
        .with("Content-Type", "application/json")
        .with("Authorization", &format!("Bearer {}", API_KEY.unwrap()));
    // Send the request
    let response = http::post(
        ENDPOINT,
        &HTTPOptions {
            headers,
            body: Some(body),
        },
    )
    .await
    .unwrap();
    // Parse the response stream
    let stream = response
        .stream()
        .string()
        .map(|s| {
            Ok(s.split("\n\n")
                .map(|s| s.trim().to_owned())
                .collect::<Vec<_>>())
        })
        .flatten()
        .filter(|s| !s.is_empty())
        .map(|s| Ok(s.strip_prefix("data:").unwrap().trim().to_owned()))
        .map(|s| {
            if s == "[DONE]" {
                return Ok(None);
            }
            let r = serde_json::from_str::<Chunk>(&s)
                .map_err(|_| Error::IO(ErrorKind::InvalidData.into()))?;
            Ok(Some(r))
        });
    stream
}

#[main]
async fn main() {
    println!("Hello, ChatGPT!");

    let _reply = http::request_access(None, 443, true, "I need access to the internet").await;

    let output: Arc<RefCell<String>> = Default::default();
    let state: Arc<RefCell<State>> = Arc::new(RefCell::new(State::Start));

    let font = Font::load("/System/Fonts/Roobert-10-Bold.pft").unwrap();
    PLAYDATE.graphics.set_font(&font);

    loop {
        let _delta = PLAYDATE.next_frame().await;

        PLAYDATE.graphics.clear(Color::White);
        PLAYDATE.system.draw_fps(vec2![0, 0]);

        let curr_state = state.borrow().clone();

        if API_KEY.is_none() {
            let text = "Please set OPENAI_API_KEY when building this app!";
            PLAYDATE.graphics.draw_text(text, vec2![20, 20]);
            continue;
        }

        if curr_state == State::Start {
            let text = format!("Press {} to start chat completion", FontIcons::A);
            PLAYDATE.graphics.draw_text(text, vec2![20, 20]);
            if Buttons::A.pushed() {
                *state.borrow_mut() = State::Loading;
                let output_clone = output.clone();
                let state_clone = state.clone();
                PLAYDATE.spawn(async move {
                    let mut chunks = chat_completion(PROMPT).await;
                    *state_clone.borrow_mut() = State::Loaded;
                    while let Some(chunk) = chunks.next().await.unwrap() {
                        let Some(delta) = chunk.choices.get(0) else {
                            continue;
                        };
                        let Some(delta) = &delta.delta.content else {
                            continue;
                        };
                        output_clone.borrow_mut().push_str(delta);
                    }
                });
            }
            continue;
        }

        if curr_state == State::Loading {
            let text = format!("Loading ...");
            PLAYDATE.graphics.draw_text(text, vec2![20, 20]);
            continue;
        }

        let text = output.borrow();
        PLAYDATE.graphics.draw_text_in_rect(
            text.as_str(),
            rect!(x: 20, y: 20, w: 360, h: 200),
            TextWrap::Word,
            TextAlign::Left,
        );
    }
}
