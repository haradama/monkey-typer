use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossbeam_channel::{unbounded, Receiver};
use std::{sync::{Arc, atomic::{AtomicBool, Ordering}}, thread, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerEvent {
    Step,
    PauseToggle,
    Panic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerKey {
    Space,
    Right,
    F12,
}

impl TriggerKey {
    pub fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "space" => Self::Space,
            "right" => Self::Right,
            "f12"   => Self::F12,
            _ => Self::Right,
        }
    }
}

pub fn poll_trigger_local(timeout_ms: u64, step_key: TriggerKey) -> Option<TriggerEvent> {
    if event::poll(Duration::from_millis(timeout_ms)).ok()? {
        if let Ok(Event::Key(KeyEvent{code, modifiers, ..})) = event::read() {
            let is_step = match (step_key, code) {
                (TriggerKey::Right, KeyCode::Right) => true,
                (TriggerKey::Space, KeyCode::Char(' ')) => true,
                _ => false,
            };
            if is_step { return Some(TriggerEvent::Step); }

            if matches!(code, KeyCode::Char('p') | KeyCode::Char('P')) && modifiers.contains(KeyModifiers::CONTROL) {
                return Some(TriggerEvent::PauseToggle);
            }

            if matches!(code, KeyCode::Esc) { return Some(TriggerEvent::Panic); }
        }
    }
    None
}

pub struct GlobalTrigger {
    rx: Receiver<TriggerEvent>,
}

impl GlobalTrigger {
    pub fn new(step_key: TriggerKey) -> Self {
        use rdev::{listen, EventType, Key};

        let (tx, rx) = unbounded();
        let ctrl_down = Arc::new(AtomicBool::new(false));
        let ctrl_flag = ctrl_down.clone();

        thread::spawn(move || {
            let _ = listen(move |event| {
                match event.event_type {
                    EventType::KeyPress(k) => {
                        if matches!(k, Key::ControlLeft | Key::ControlRight) {
                            ctrl_flag.store(true, Ordering::Relaxed);
                            return;
                        }

                        let is_step = match (step_key, k) {
                            (TriggerKey::F12,  Key::F12)        => true,
                            (TriggerKey::Space, Key::Space)      => true,
                            (TriggerKey::Right, Key::RightArrow) => true,
                            _ => false,
                        };
                        if is_step {
                            let _ = tx.send(TriggerEvent::Step);
                            return;
                        }

                        if matches!(k, Key::KeyP) && ctrl_flag.load(Ordering::Relaxed) {
                            let _ = tx.send(TriggerEvent::PauseToggle);
                            return;
                        }

                        if matches!(k, Key::Escape) {
                            let _ = tx.send(TriggerEvent::Panic);
                            return;
                        }
                    }
                    EventType::KeyRelease(k) => {
                        if matches!(k, Key::ControlLeft | Key::ControlRight) {
                            ctrl_flag.store(false, Ordering::Relaxed);
                        }
                    }
                    _ => {}
                }
            });
        });

        Self { rx }
    }

    pub fn try_recv(&self) -> Option<TriggerEvent> {
        self.rx.try_recv().ok()
    }
}
