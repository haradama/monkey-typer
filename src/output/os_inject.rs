use crate::prelude::*;
use enigo::{Enigo, KeyboardControllable, Key};
use super::injector::OutputInjector;

pub struct OsInjector {
    enigo: Enigo,
}

impl OsInjector {
    pub fn new() -> Self { Self { enigo: Enigo::new() } }
}

impl OutputInjector for OsInjector {
    fn send_char(&mut self, ch: char) -> Result<()> {
        match ch {
            '\n' | '\r' => self.enigo.key_click(Key::Return),
            '\t'        => self.enigo.key_click(Key::Tab),
            _           => self.enigo.key_sequence(&ch.to_string()),
        }
        Ok(())
    }

    fn backspace(&mut self) -> Result<()> {
        self.enigo.key_click(Key::Backspace);
        Ok(())
    }
}
