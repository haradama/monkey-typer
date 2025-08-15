use crate::{prelude::*, output::injector::OutputInjector};
use super::playhead::Playhead;
use crate::format::tks_json::Action;

pub struct Engine {
    pub actions: Vec<Action>,
    pub _markers: Vec<(usize, String)>,
    pub head: Playhead,
}

impl Engine {
    pub fn new(actions: Vec<Action>, _markers: Vec<(usize, String)>) -> Self {
        let end = actions.len();
        Self { actions, _markers, head: Playhead::new(end) }
    }

    pub fn step<I: OutputInjector>(&mut self, inj: &mut I) -> Result<()> {
        if self.head.paused || self.head.is_eof() { return Ok(()); }
        let act = self.actions[self.head.pos];
        match act {
            Action::Char(ch) => {
                debug!("step pos={} char={:?}", self.head.pos, ch);
                inj.send_char(ch)?;
            }
            Action::Backspace => {
                debug!("step pos={} backspace", self.head.pos);
                inj.backspace()?;
            }
        }
        self.head.pos += 1;
        Ok(())
    }

    pub fn toggle_pause(&mut self) { self.head.paused = !self.head.paused; }
}
