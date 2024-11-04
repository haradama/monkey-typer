use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

/// Enum representing various key types
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum Key {
    Alt,
    AltGr,
    Backspace,
    CapsLock,
    ControlLeft,
    ControlRight,
    Delete,
    DownArrow,
    End,
    Escape,
    F(u8),
    Home,
    LeftArrow,
    MetaLeft,
    MetaRight,
    PageDown,
    PageUp,
    Return,
    RightArrow,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    UpArrow,
    PrintScreen,
    ScrollLock,
    Pause,
    NumLock,
    BackQuote,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equal,
    KeyQ,
    KeyW,
    KeyE,
    KeyR,
    KeyT,
    KeyY,
    KeyU,
    KeyI,
    KeyO,
    KeyP,
    LeftBracket,
    RightBracket,
    KeyA,
    KeyS,
    KeyD,
    KeyF,
    KeyG,
    KeyH,
    KeyJ,
    KeyK,
    KeyL,
    SemiColon,
    Quote,
    BackSlash,
    IntlBackslash,
    KeyZ,
    KeyX,
    KeyC,
    KeyV,
    KeyB,
    KeyN,
    KeyM,
    Comma,
    Dot,
    Slash,
    Insert,
    KpReturn,
    KpMinus,
    KpPlus,
    KpMultiply,
    KpDivide,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDelete,
    Function,
    Unknown(u32),
}

/// Structure representing a key event
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyEvent {
    pub key: Key,
    pub text: Option<String>, // Holds the input text if available
}

impl KeyEvent {
    /// Converts an rdev::Key event to KeyEvent
    pub fn from_rdev_event(event: &rdev::Event) -> Option<Self> {
        use rdev::{EventType, Key as RdevKey};
        if let EventType::KeyPress(key) = event.event_type {
            let key = match key {
                RdevKey::Alt => Key::Alt,
                RdevKey::AltGr => Key::AltGr,
                RdevKey::Backspace => Key::Backspace,
                RdevKey::CapsLock => Key::CapsLock,
                RdevKey::ControlLeft => Key::ControlLeft,
                RdevKey::ControlRight => Key::ControlRight,
                RdevKey::Delete => Key::Delete,
                RdevKey::DownArrow => Key::DownArrow,
                RdevKey::End => Key::End,
                RdevKey::Escape => Key::Escape,
                RdevKey::F1 => Key::F(1),
                RdevKey::F2 => Key::F(2),
                RdevKey::F3 => Key::F(3),
                RdevKey::F4 => Key::F(4),
                RdevKey::F5 => Key::F(5),
                RdevKey::F6 => Key::F(6),
                RdevKey::F7 => Key::F(7),
                RdevKey::F8 => Key::F(8),
                RdevKey::F9 => Key::F(9),
                RdevKey::F10 => Key::F(10),
                RdevKey::F11 => Key::F(11),
                RdevKey::F12 => Key::F(12),
                RdevKey::Home => Key::Home,
                RdevKey::LeftArrow => Key::LeftArrow,
                RdevKey::MetaLeft => Key::MetaLeft,
                RdevKey::MetaRight => Key::MetaRight,
                RdevKey::PageDown => Key::PageDown,
                RdevKey::PageUp => Key::PageUp,
                RdevKey::Return => Key::Return,
                RdevKey::RightArrow => Key::RightArrow,
                RdevKey::ShiftLeft => Key::ShiftLeft,
                RdevKey::ShiftRight => Key::ShiftRight,
                RdevKey::Space => Key::Space,
                RdevKey::Tab => Key::Tab,
                RdevKey::UpArrow => Key::UpArrow,
                RdevKey::PrintScreen => Key::PrintScreen,
                RdevKey::ScrollLock => Key::ScrollLock,
                RdevKey::Pause => Key::Pause,
                RdevKey::NumLock => Key::NumLock,
                RdevKey::BackQuote => Key::BackQuote,
                RdevKey::Num1 => Key::Num1,
                RdevKey::Num2 => Key::Num2,
                RdevKey::Num3 => Key::Num3,
                RdevKey::Num4 => Key::Num4,
                RdevKey::Num5 => Key::Num5,
                RdevKey::Num6 => Key::Num6,
                RdevKey::Num7 => Key::Num7,
                RdevKey::Num8 => Key::Num8,
                RdevKey::Num9 => Key::Num9,
                RdevKey::Num0 => Key::Num0,
                RdevKey::Minus => Key::Minus,
                RdevKey::Equal => Key::Equal,
                RdevKey::KeyQ => Key::KeyQ,
                RdevKey::KeyW => Key::KeyW,
                RdevKey::KeyE => Key::KeyE,
                RdevKey::KeyR => Key::KeyR,
                RdevKey::KeyT => Key::KeyT,
                RdevKey::KeyY => Key::KeyY,
                RdevKey::KeyU => Key::KeyU,
                RdevKey::KeyI => Key::KeyI,
                RdevKey::KeyO => Key::KeyO,
                RdevKey::KeyP => Key::KeyP,
                RdevKey::LeftBracket => Key::LeftBracket,
                RdevKey::RightBracket => Key::RightBracket,
                RdevKey::KeyA => Key::KeyA,
                RdevKey::KeyS => Key::KeyS,
                RdevKey::KeyD => Key::KeyD,
                RdevKey::KeyF => Key::KeyF,
                RdevKey::KeyG => Key::KeyG,
                RdevKey::KeyH => Key::KeyH,
                RdevKey::KeyJ => Key::KeyJ,
                RdevKey::KeyK => Key::KeyK,
                RdevKey::KeyL => Key::KeyL,
                RdevKey::SemiColon => Key::SemiColon,
                RdevKey::Quote => Key::Quote,
                RdevKey::BackSlash => Key::BackSlash,
                RdevKey::IntlBackslash => Key::IntlBackslash,
                RdevKey::KeyZ => Key::KeyZ,
                RdevKey::KeyX => Key::KeyX,
                RdevKey::KeyC => Key::KeyC,
                RdevKey::KeyV => Key::KeyV,
                RdevKey::KeyB => Key::KeyB,
                RdevKey::KeyN => Key::KeyN,
                RdevKey::KeyM => Key::KeyM,
                RdevKey::Comma => Key::Comma,
                RdevKey::Dot => Key::Dot,
                RdevKey::Slash => Key::Slash,
                RdevKey::Insert => Key::Insert,
                RdevKey::KpReturn => Key::KpReturn,
                RdevKey::KpMinus => Key::KpMinus,
                RdevKey::KpPlus => Key::KpPlus,
                RdevKey::KpMultiply => Key::KpMultiply,
                RdevKey::KpDivide => Key::KpDivide,
                RdevKey::Kp0 => Key::Kp0,
                RdevKey::Kp1 => Key::Kp1,
                RdevKey::Kp2 => Key::Kp2,
                RdevKey::Kp3 => Key::Kp3,
                RdevKey::Kp4 => Key::Kp4,
                RdevKey::Kp5 => Key::Kp5,
                RdevKey::Kp6 => Key::Kp6,
                RdevKey::Kp7 => Key::Kp7,
                RdevKey::Kp8 => Key::Kp8,
                RdevKey::Kp9 => Key::Kp9,
                RdevKey::KpDelete => Key::KpDelete,
                RdevKey::Function => Key::Function,
                RdevKey::Unknown(code) => Key::Unknown(code),
            };

            // Capture the text associated with the event (if any)
            let text = event.name.clone();

            Some(KeyEvent { key, text })
        } else {
            None
        }
    }
}

/// Writer to serialize and write key events sequentially
pub struct KeyEventWriter {
    writer: BufWriter<File>,
}

impl KeyEventWriter {
    /// Create a new KeyEventWriter
    pub fn open(file_path: &str) -> Result<Self, std::io::Error> {
        let file = File::create(file_path)?;
        let writer = BufWriter::new(file);
        Ok(KeyEventWriter { writer })
    }

    /// Serialize and write a key event
    pub fn write_event(&mut self, event: &KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
        serde_json::to_writer(&mut self.writer, event)?;
        self.writer.write_all(b"\n")?;
        self.writer.flush()?;
        Ok(())
    }
}

/// Reader to deserialize and read key events sequentially
pub struct KeyEventReader {
    stream:
        serde_json::StreamDeserializer<'static, serde_json::de::IoRead<BufReader<File>>, KeyEvent>,
}

impl KeyEventReader {
    /// Create a new KeyEventReader
    pub fn open(file_path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let deserializer = Deserializer::from_reader(reader);
        let stream = deserializer.into_iter::<KeyEvent>();
        Ok(KeyEventReader { stream })
    }

    /// Deserialize and read the next key event
    pub fn read_event(&mut self) -> Option<Result<KeyEvent, serde_json::Error>> {
        self.stream.next()
    }
}
