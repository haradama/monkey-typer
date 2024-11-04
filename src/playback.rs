use std::collections::HashSet;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::data::{KeyEvent, KeyEventReader, KeyEventType};
use rdev::{listen, simulate, EventType, Key as RdevKey, SimulateError};

/// Function to play a recorded session from a file
pub fn play_session(session_file: &str) -> Result<(), Box<dyn Error>> {
    // Open the file for playback
    let reader = Arc::new(Mutex::new(KeyEventReader::open(session_file)?));

    println!("Playback started. Press any key to replay events. Press 'Escape' to quit.");

    // Flag to control the listener loop
    let running = Arc::new(AtomicBool::new(true));

    // Set to keep track of currently pressed keys
    let pressed_keys = Arc::new(Mutex::new(HashSet::new()));

    // Clone references for the listener closure
    let reader_clone = Arc::clone(&reader);
    let running_clone = Arc::clone(&running);
    let pressed_keys_clone = Arc::clone(&pressed_keys);

    // Start listening for global key events
    let listen_result = listen(move |event| {
        if !running_clone.load(Ordering::SeqCst) {
            return;
        }

        match event.event_type {
            EventType::KeyPress(rdev_key) => {
                let mut pressed_keys = pressed_keys_clone.lock().unwrap();
                // Check if this key is already pressed
                if pressed_keys.contains(&rdev_key) {
                    // Key is already pressed; ignore to prevent multiple triggers
                    return;
                }
                // Mark the key as pressed
                pressed_keys.insert(rdev_key);

                // Check if the user wants to quit (e.g., pressing 'Escape')
                if rdev_key == RdevKey::Escape {
                    running_clone.store(false, Ordering::SeqCst);
                    return;
                }

                // Read the next recorded key event
                let mut reader = reader_clone.lock().unwrap();
                match reader.read_event() {
                    Some(Ok(recorded_event)) => {
                        // Simulate the recorded key event
                        if let Err(err) = simulate_key_event(&recorded_event) {
                            eprintln!("Error occurred while simulating key event: {:?}", err);
                            running_clone.store(false, Ordering::SeqCst);
                        }
                    }
                    Some(Err(err)) => {
                        eprintln!("Error occurred while reading key event: {:?}", err);
                        running_clone.store(false, Ordering::SeqCst);
                    }
                    None => {
                        println!("\nPlayback finished.");
                        running_clone.store(false, Ordering::SeqCst);
                    }
                }
            }
            EventType::KeyRelease(rdev_key) => {
                let mut pressed_keys = pressed_keys_clone.lock().unwrap();
                // Remove the key from the pressed keys set
                pressed_keys.remove(&rdev_key);
            }
            _ => {}
        }
    });

    // Handle any errors from the listener
    if let Err(err) = listen_result {
        eprintln!("Error occurred while listening for events: {:?}", err);
    }

    Ok(())
}

/// Simulate the given key event
fn simulate_key_event(event: &KeyEvent) -> Result<(), SimulateError> {
    // Map our KeyEvent to rdev::Key
    let rdev_key = key_to_rdev_key(event.key);

    // Simulate the event based on its type
    let event_type = match event.event_type {
        KeyEventType::Press => EventType::KeyPress(rdev_key),
        KeyEventType::Release => EventType::KeyRelease(rdev_key),
    };

    // Simulate the key event
    simulate(&event_type)
}

/// Map our Key enum to rdev::Key
fn key_to_rdev_key(key: crate::data::Key) -> RdevKey {
    use crate::data::Key::*;
    use RdevKey as RK;

    match key {
        Alt => RK::Alt,
        AltGr => RK::AltGr,
        Backspace => RK::Backspace,
        CapsLock => RK::CapsLock,
        ControlLeft => RK::ControlLeft,
        ControlRight => RK::ControlRight,
        Delete => RK::Delete,
        DownArrow => RK::DownArrow,
        End => RK::End,
        Escape => RK::Escape,
        F(n) => match n {
            1 => RK::F1,
            2 => RK::F2,
            3 => RK::F3,
            4 => RK::F4,
            5 => RK::F5,
            6 => RK::F6,
            7 => RK::F7,
            8 => RK::F8,
            9 => RK::F9,
            10 => RK::F10,
            11 => RK::F11,
            12 => RK::F12,
            _ => RK::Unknown(0),
        },
        Home => RK::Home,
        LeftArrow => RK::LeftArrow,
        MetaLeft => RK::MetaLeft,
        MetaRight => RK::MetaRight,
        PageDown => RK::PageDown,
        PageUp => RK::PageUp,
        Return => RK::Return,
        RightArrow => RK::RightArrow,
        ShiftLeft => RK::ShiftLeft,
        ShiftRight => RK::ShiftRight,
        Space => RK::Space,
        Tab => RK::Tab,
        UpArrow => RK::UpArrow,
        PrintScreen => RK::PrintScreen,
        ScrollLock => RK::ScrollLock,
        Pause => RK::Pause,
        NumLock => RK::NumLock,
        BackQuote => RK::BackQuote,
        Num1 => RK::Num1,
        Num2 => RK::Num2,
        Num3 => RK::Num3,
        Num4 => RK::Num4,
        Num5 => RK::Num5,
        Num6 => RK::Num6,
        Num7 => RK::Num7,
        Num8 => RK::Num8,
        Num9 => RK::Num9,
        Num0 => RK::Num0,
        Minus => RK::Minus,
        Equal => RK::Equal,
        KeyQ => RK::KeyQ,
        KeyW => RK::KeyW,
        KeyE => RK::KeyE,
        KeyR => RK::KeyR,
        KeyT => RK::KeyT,
        KeyY => RK::KeyY,
        KeyU => RK::KeyU,
        KeyI => RK::KeyI,
        KeyO => RK::KeyO,
        KeyP => RK::KeyP,
        LeftBracket => RK::LeftBracket,
        RightBracket => RK::RightBracket,
        KeyA => RK::KeyA,
        KeyS => RK::KeyS,
        KeyD => RK::KeyD,
        KeyF => RK::KeyF,
        KeyG => RK::KeyG,
        KeyH => RK::KeyH,
        KeyJ => RK::KeyJ,
        KeyK => RK::KeyK,
        KeyL => RK::KeyL,
        SemiColon => RK::SemiColon,
        Quote => RK::Quote,
        BackSlash => RK::BackSlash,
        IntlBackslash => RK::IntlBackslash,
        KeyZ => RK::KeyZ,
        KeyX => RK::KeyX,
        KeyC => RK::KeyC,
        KeyV => RK::KeyV,
        KeyB => RK::KeyB,
        KeyN => RK::KeyN,
        KeyM => RK::KeyM,
        Comma => RK::Comma,
        Dot => RK::Dot,
        Slash => RK::Slash,
        Insert => RK::Insert,
        KpReturn => RK::KpReturn,
        KpMinus => RK::KpMinus,
        KpPlus => RK::KpPlus,
        KpMultiply => RK::KpMultiply,
        KpDivide => RK::KpDivide,
        Kp0 => RK::Kp0,
        Kp1 => RK::Kp1,
        Kp2 => RK::Kp2,
        Kp3 => RK::Kp3,
        Kp4 => RK::Kp4,
        Kp5 => RK::Kp5,
        Kp6 => RK::Kp6,
        Kp7 => RK::Kp7,
        Kp8 => RK::Kp8,
        Kp9 => RK::Kp9,
        KpDelete => RK::KpDelete,
        Function => RK::Function,
        Unknown(code) => RK::Unknown(code),
    }
}
