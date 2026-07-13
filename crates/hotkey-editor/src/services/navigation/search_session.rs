use dioxus::prelude::*;

/// The debounced search-typing session. Typing in the unit list must not flood the
/// browser history with one entry per keystroke: the first keystroke pushes a history
/// entry and opens a session; every keystroke while the session is open replaces instead;
/// the session closes 500 ms after the last keystroke, so the next burst pushes afresh.
///
/// `active` is whether a session is currently open; `generation` is bumped on every
/// keystroke so a stale 500 ms timer (one whose keystroke was superseded) closes nothing.
/// The signals are owned by the shell and seeded fresh (closed) on entry.
#[derive(Clone, Copy, PartialEq)]
pub struct SearchSession {
    active: Signal<bool>,
    generation: Signal<u32>,
}

impl SearchSession {
    pub fn new(active: Signal<bool>, generation: Signal<u32>) -> Self {
        Self { active, generation }
    }

    pub fn active(&self) -> Signal<bool> {
        self.active
    }

    pub fn generation(&self) -> Signal<u32> {
        self.generation
    }
}
