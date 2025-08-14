use crate::prelude::*;

pub trait OutputInjector {
    fn send_char(&mut self, ch: char) -> Result<()>;
    fn backspace(&mut self) -> Result<()>;

    fn send_str(&mut self, s: &str) -> Result<()> {
        for ch in s.chars() { self.send_char(ch)?; }
        Ok(())
    }
}