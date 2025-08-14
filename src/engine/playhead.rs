#[derive(Debug, Clone)]
pub struct Playhead {
    pub pos: usize,
    pub end: usize,
    pub paused: bool,
}

impl Playhead {
    pub fn new(end: usize) -> Self {
        Self { pos: 0, end, paused: false }
    }
    #[inline]
    pub fn is_eof(&self) -> bool { self.pos >= self.end }
}