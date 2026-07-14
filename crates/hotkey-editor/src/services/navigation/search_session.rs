use dioxus::prelude::*;

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
