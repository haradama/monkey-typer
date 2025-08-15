use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossbeam_channel::Receiver;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerEvent {
    Start,
    Step { from_alpha: bool },
    PauseToggle,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerKey { Space, Right, F11, F12, CtrlP, CtrlQ, CtrlS }

impl TriggerKey {
    pub fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "space"   => Self::Space,
            "right"   => Self::Right,
            "f11"     => Self::F11,
            "f12"     => Self::F12,
            "ctrl+p"  => Self::CtrlP,
            "ctrl+q"  => Self::CtrlQ,
            "ctrl+s"  => Self::CtrlS,
            _ => Self::Right,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hotkeys {
    pub start: TriggerKey,
    pub step:  TriggerKey,
    pub pause: TriggerKey,
    pub exit:  TriggerKey,
}
impl Hotkeys {
    pub fn from_strings(step: &str, pause: &str, start: &str, exit: &str) -> Self {
        Self {
            step:  TriggerKey::parse(step),
            pause: TriggerKey::parse(pause),
            start: TriggerKey::parse(start),
            exit:  TriggerKey::parse(exit),
        }
    }
}

pub fn poll_trigger_local(timeout_ms: u64, keys: &Hotkeys) -> Option<TriggerEvent> {
    if event::poll(Duration::from_millis(timeout_ms)).ok()? {
        if let Ok(Event::Key(KeyEvent{code, modifiers, kind, ..})) = event::read() {
            if !matches!(kind, KeyEventKind::Press) { return None; }

            let is_start = match (keys.start, code, modifiers.contains(KeyModifiers::CONTROL)) {
                (TriggerKey::CtrlS, KeyCode::Char('s' | 'S'), true) => true,
                (TriggerKey::F11,   KeyCode::F(11),           _   ) => true,
                _ => false,
            };
            if is_start { return Some(TriggerEvent::Start); }

            let is_exit = match (keys.exit, code, modifiers.contains(KeyModifiers::CONTROL)) {
                (TriggerKey::CtrlQ, KeyCode::Char('q' | 'Q'), true) => true,
                _ => false,
            };
            if is_exit { return Some(TriggerEvent::Exit); }

            let is_pause = match (keys.pause, code, modifiers.contains(KeyModifiers::CONTROL)) {
                (TriggerKey::CtrlP, KeyCode::Char('p' | 'P'), true) => true,
                _ => false,
            };
            if is_pause { return Some(TriggerEvent::PauseToggle); }

            let is_step_single = match (keys.step, code) {
                (TriggerKey::Right, KeyCode::Right)    => true,
                (TriggerKey::Space, KeyCode::Char(' '))=> true,
                (TriggerKey::F12,   KeyCode::F(12))    => true,
                _ => false,
            };
            if is_step_single { return Some(TriggerEvent::Step { from_alpha: false }); }

            if let KeyCode::Char(c) = code {
                if c.is_ascii_alphabetic() && !modifiers.contains(KeyModifiers::CONTROL) {
                    return Some(TriggerEvent::Step { from_alpha: false });
                }
            }
        }
    }
    None
}

#[cfg(target_os = "macos")]
pub struct GlobalTrigger { rx: Receiver<TriggerEvent> }

#[cfg(target_os = "macos")]
impl GlobalTrigger {
    pub fn new(keys: Hotkeys) -> Self {
        use core_foundation::runloop::CFRunLoop;
        use core_graphics::event::{
            CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement,
            CGEventType, EventField, KeyCode, CGEventFlags, CallbackResult,
        };
        use crossbeam_channel::unbounded;
        use std::sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        };

        let (tx, rx) = unbounded::<TriggerEvent>();

        let armed = Arc::new(AtomicBool::new(false));
        let armed_cb = armed.clone();

        let my_pid: u32 = std::process::id();

        std::thread::spawn(move || {
            let events = vec![CGEventType::KeyDown];

            let _tap = CGEventTap::with_enabled(
                CGEventTapLocation::HID,
                CGEventTapPlacement::HeadInsertEventTap,
                CGEventTapOptions::Default,
                events,
                move |_proxy, etype, event| {
                    if !matches!(etype, CGEventType::KeyDown) {
                        return CallbackResult::Keep;
                    }

                    let autorep = event.get_integer_value_field(EventField::KEYBOARD_EVENT_AUTOREPEAT) != 0;
                    if autorep {
                        return CallbackResult::Drop;
                    }

                    let src_pid = event.get_integer_value_field(EventField::EVENT_SOURCE_UNIX_PROCESS_ID) as u32;
                    if src_pid == my_pid {
                        return CallbackResult::Keep;
                    }

                    let keycode = event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE) as u16;
                    let flags   = event.get_flags();
                    let ctrl    = flags.contains(CGEventFlags::CGEventFlagControl);

                    if ctrl && keycode == KeyCode::ANSI_S && matches!(keys.start, TriggerKey::CtrlS) {
                        let _ = tx.send(TriggerEvent::Start);
                        armed_cb.store(true, Ordering::Relaxed);
                        return CallbackResult::Drop;
                    }
                    if ctrl && keycode == KeyCode::ANSI_P && matches!(keys.pause, TriggerKey::CtrlP) {
                        let _ = tx.send(TriggerEvent::PauseToggle);
                        return CallbackResult::Drop;
                    }
                    if ctrl && keycode == KeyCode::ANSI_Q && matches!(keys.exit, TriggerKey::CtrlQ) {
                        let _ = tx.send(TriggerEvent::Exit);
                        return CallbackResult::Drop;
                    }

                    if !armed_cb.load(Ordering::Relaxed) {
                        return CallbackResult::Keep;
                    }

                    let step_single = match keys.step {
                        TriggerKey::F12   => keycode == KeyCode::F12,
                        TriggerKey::Space => keycode == KeyCode::SPACE,
                        TriggerKey::Right => keycode == KeyCode::RIGHT_ARROW,
                        _ => false,
                    };
                    if step_single {
                        let _ = tx.send(TriggerEvent::Step { from_alpha: false });
                        return CallbackResult::Drop;
                    }

                    if !ctrl && is_alpha_keycode(keycode) {
                        let _ = tx.send(TriggerEvent::Step { from_alpha: false });
                        return CallbackResult::Drop;
                    }

                    CallbackResult::Keep
                },
                || CFRunLoop::run_current(),
            ).expect("Failed to install CGEventTap (Accessibility / Input Monitoring を許可してください)");

        });

        Self { rx }
    }

    pub fn try_recv(&self) -> Option<TriggerEvent> { self.rx.try_recv().ok() }
}

#[cfg(target_os = "macos")]
#[inline]
fn is_alpha_keycode(code: u16) -> bool {
    use core_graphics::event::KeyCode;
    matches!(code,
        KeyCode::ANSI_A | KeyCode::ANSI_B | KeyCode::ANSI_C | KeyCode::ANSI_D |
        KeyCode::ANSI_E | KeyCode::ANSI_F | KeyCode::ANSI_G | KeyCode::ANSI_H |
        KeyCode::ANSI_I | KeyCode::ANSI_J | KeyCode::ANSI_K | KeyCode::ANSI_L |
        KeyCode::ANSI_M | KeyCode::ANSI_N | KeyCode::ANSI_O | KeyCode::ANSI_P |
        KeyCode::ANSI_Q | KeyCode::ANSI_R | KeyCode::ANSI_S | KeyCode::ANSI_T |
        KeyCode::ANSI_U | KeyCode::ANSI_V | KeyCode::ANSI_W | KeyCode::ANSI_X |
        KeyCode::ANSI_Y | KeyCode::ANSI_Z
    )
}

#[cfg(not(target_os = "macos"))]
pub struct GlobalTrigger { rx: Receiver<TriggerEvent> }
#[cfg(not(target_os = "macos"))]
impl GlobalTrigger {
    pub fn new(_keys: Hotkeys) -> Self {
        let (_tx, rx) = unbounded();
        eprintln!("GlobalTrigger is only available on macOS.");
        Self { rx }
    }
    pub fn try_recv(&self) -> Option<TriggerEvent> { self.rx.try_recv().ok() }
}
