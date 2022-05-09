use inputbot::{KeybdKey::*, MouseButton::*, *};
// use std::{thread::sleep, time::Duration};

fn passthrough_key(event: char) {
    println!("{}", event);
}

fn main() {
    // // Autorun for videogames.
    // NumLockKey.bind(|| {
    //     while NumLockKey.is_toggled() {
    //         LShiftKey.press();
    //         WKey.press();
    //         sleep(Duration::from_millis(50));
    //         WKey.release();
    //         LShiftKey.release();
    //     }
    // });

    // // Rapidfire for videogames.
    // RightButton.bind(|| {
    //     while RightButton.is_pressed() {
    //         LeftButton.press();
    //         sleep(Duration::from_millis(200));
    //         LeftButton.release();
    //     }
    // });

    LeftButton.bind(|| {
        println!("Left Press");
    });

    // // Send a key sequence.
    // RKey.bind(|| KeySequence("Sample text").send());

    // // Move mouse.
    // QKey.bind(|| MouseCursor::move_rel(10, 10));

//LeftButton
// MiddleButton
// RightButton
// X1Button
// X2Button
// OtherButton(u32)

// BackspaceKey
// TabKey
// EnterKey
// EscapeKey
// SpaceKey
// HomeKey
// LeftKey
// UpKey
// RightKey
// DownKey
// InsertKey
// DeleteKey
// Numrow0Key
// Numrow1Key
// Numrow2Key
// Numrow3Key
// Numrow4Key
// Numrow5Key
// Numrow6Key
// Numrow7Key
// Numrow8Key
// Numrow9Key

    AKey.bind(|| { passthrough_key('a') });
    BKey.bind(|| { passthrough_key('b') });
    CKey.bind(|| { passthrough_key('c') });
    DKey.bind(|| { passthrough_key('d') });
    EKey.bind(|| { passthrough_key('e') });
    FKey.bind(|| { passthrough_key('f') });
    GKey.bind(|| { passthrough_key('g') });
    HKey.bind(|| { passthrough_key('h') });
    IKey.bind(|| { passthrough_key('i') });
    JKey.bind(|| { passthrough_key('j') });
    KKey.bind(|| { passthrough_key('k') });
    LKey.bind(|| { passthrough_key('l') });
    MKey.bind(|| { passthrough_key('m') });
    NKey.bind(|| { passthrough_key('n') });
    OKey.bind(|| { passthrough_key('o') });
    PKey.bind(|| { passthrough_key('p') });
    QKey.bind(|| { passthrough_key('q') });
    RKey.bind(|| { passthrough_key('r') });
    SKey.bind(|| { passthrough_key('s') });
    TKey.bind(|| { passthrough_key('t') });
    UKey.bind(|| { passthrough_key('u') });
    VKey.bind(|| { passthrough_key('v') });
    WKey.bind(|| { passthrough_key('w') });
    XKey.bind(|| { passthrough_key('x') });
    YKey.bind(|| { passthrough_key('y') });
    ZKey.bind(|| { passthrough_key('z') });

    handle_input_events();
}