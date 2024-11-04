use rdev::EventType;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::time::{SystemTime, UNIX_EPOCH};

/// Enum representing the type of key event
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum KeyEventType {
    Press,
    Release,
}

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
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyEvent {
    pub event_type: KeyEventType,
    pub key: Key,
    pub text: Option<String>, // Holds the input text if available
    pub timestamp: u128,      // Time in milliseconds since UNIX_EPOCH
}

impl KeyEvent {
    /// Converts an rdev::Event to KeyEvent
    pub fn from_rdev_event(event: &rdev::Event) -> Option<Self> {
        let event_type = match event.event_type {
            EventType::KeyPress(_) => KeyEventType::Press,
            EventType::KeyRelease(_) => KeyEventType::Release,
            _ => return None, // Ignore other event types
        };

        let key = match event.event_type {
            EventType::KeyPress(key) | EventType::KeyRelease(key) => {
                // Map rdev::Key to our Key enum
                rdev_key_to_key(key)
            }
            _ => return None,
        };

        // Capture the text associated with the event (if any)
        let text = event.name.clone();

        // Get the timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        Some(KeyEvent {
            event_type,
            key,
            text,
            timestamp,
        })
    }
}

/// Map rdev::Key to our Key enum
fn rdev_key_to_key(key: rdev::Key) -> Key {
    use rdev::Key as RdevKey;
    use Key::*;

    match key {
        RdevKey::Alt => Alt,
        RdevKey::AltGr => AltGr,
        RdevKey::Backspace => Backspace,
        RdevKey::CapsLock => CapsLock,
        RdevKey::ControlLeft => ControlLeft,
        RdevKey::ControlRight => ControlRight,
        RdevKey::Delete => Delete,
        RdevKey::DownArrow => DownArrow,
        RdevKey::End => End,
        RdevKey::Escape => Escape,
        RdevKey::F1 => F(1),
        RdevKey::F2 => F(2),
        RdevKey::F3 => F(3),
        RdevKey::F4 => F(4),
        RdevKey::F5 => F(5),
        RdevKey::F6 => F(6),
        RdevKey::F7 => F(7),
        RdevKey::F8 => F(8),
        RdevKey::F9 => F(9),
        RdevKey::F10 => F(10),
        RdevKey::F11 => F(11),
        RdevKey::F12 => F(12),
        RdevKey::Home => Home,
        RdevKey::LeftArrow => LeftArrow,
        RdevKey::MetaLeft => MetaLeft,
        RdevKey::MetaRight => MetaRight,
        RdevKey::PageDown => PageDown,
        RdevKey::PageUp => PageUp,
        RdevKey::Return => Return,
        RdevKey::RightArrow => RightArrow,
        RdevKey::ShiftLeft => ShiftLeft,
        RdevKey::ShiftRight => ShiftRight,
        RdevKey::Space => Space,
        RdevKey::Tab => Tab,
        RdevKey::UpArrow => UpArrow,
        RdevKey::PrintScreen => PrintScreen,
        RdevKey::ScrollLock => ScrollLock,
        RdevKey::Pause => Pause,
        RdevKey::NumLock => NumLock,
        RdevKey::BackQuote => BackQuote,
        RdevKey::Num1 => Num1,
        RdevKey::Num2 => Num2,
        RdevKey::Num3 => Num3,
        RdevKey::Num4 => Num4,
        RdevKey::Num5 => Num5,
        RdevKey::Num6 => Num6,
        RdevKey::Num7 => Num7,
        RdevKey::Num8 => Num8,
        RdevKey::Num9 => Num9,
        RdevKey::Num0 => Num0,
        RdevKey::Minus => Minus,
        RdevKey::Equal => Equal,
        RdevKey::KeyQ => KeyQ,
        RdevKey::KeyW => KeyW,
        RdevKey::KeyE => KeyE,
        RdevKey::KeyR => KeyR,
        RdevKey::KeyT => KeyT,
        RdevKey::KeyY => KeyY,
        RdevKey::KeyU => KeyU,
        RdevKey::KeyI => KeyI,
        RdevKey::KeyO => KeyO,
        RdevKey::KeyP => KeyP,
        RdevKey::LeftBracket => LeftBracket,
        RdevKey::RightBracket => RightBracket,
        RdevKey::KeyA => KeyA,
        RdevKey::KeyS => KeyS,
        RdevKey::KeyD => KeyD,
        RdevKey::KeyF => KeyF,
        RdevKey::KeyG => KeyG,
        RdevKey::KeyH => KeyH,
        RdevKey::KeyJ => KeyJ,
        RdevKey::KeyK => KeyK,
        RdevKey::KeyL => KeyL,
        RdevKey::SemiColon => SemiColon,
        RdevKey::Quote => Quote,
        RdevKey::BackSlash => BackSlash,
        RdevKey::IntlBackslash => IntlBackslash,
        RdevKey::KeyZ => KeyZ,
        RdevKey::KeyX => KeyX,
        RdevKey::KeyC => KeyC,
        RdevKey::KeyV => KeyV,
        RdevKey::KeyB => KeyB,
        RdevKey::KeyN => KeyN,
        RdevKey::KeyM => KeyM,
        RdevKey::Comma => Comma,
        RdevKey::Dot => Dot,
        RdevKey::Slash => Slash,
        RdevKey::Insert => Insert,
        RdevKey::KpReturn => KpReturn,
        RdevKey::KpMinus => KpMinus,
        RdevKey::KpPlus => KpPlus,
        RdevKey::KpMultiply => KpMultiply,
        RdevKey::KpDivide => KpDivide,
        RdevKey::Kp0 => Kp0,
        RdevKey::Kp1 => Kp1,
        RdevKey::Kp2 => Kp2,
        RdevKey::Kp3 => Kp3,
        RdevKey::Kp4 => Kp4,
        RdevKey::Kp5 => Kp5,
        RdevKey::Kp6 => Kp6,
        RdevKey::Kp7 => Kp7,
        RdevKey::Kp8 => Kp8,
        RdevKey::Kp9 => Kp9,
        RdevKey::KpDelete => KpDelete,
        RdevKey::Function => Function,
        RdevKey::Unknown(code) => Unknown(code),
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
