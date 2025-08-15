use crate::prelude::*;

pub trait OutputInjector {
    fn send_char(&mut self, ch: char) -> Result<()>;
    fn backspace(&mut self) -> Result<()>;
}