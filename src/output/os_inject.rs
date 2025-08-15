use super::injector::OutputInjector;
use crate::prelude::*;
use enigo::Keyboard;
use enigo::{Direction, Enigo, Key, Settings};

pub struct OsInjector {
    enigo: Enigo,
}

impl OsInjector {
    pub fn new() -> Self {
        let enigo = Enigo::new(&Settings::default()).expect("failed to create Enigo");
        Self { enigo }
    }
}

impl OutputInjector for OsInjector {
    fn send_char(&mut self, ch: char) -> Result<()> {
        match ch {
            '\n' | '\r' => self.enigo.key(Key::Return, Direction::Click)?,
            '\t' => self.enigo.key(Key::Tab, Direction::Click)?,
            _ => {
                let s = ch.to_string();
                self.enigo.text(&s)?;
            }
        }
        Ok(())
    }

    fn backspace(&mut self) -> Result<()> {
        self.enigo.key(Key::Backspace, Direction::Click)?;
        Ok(())
    }
}
