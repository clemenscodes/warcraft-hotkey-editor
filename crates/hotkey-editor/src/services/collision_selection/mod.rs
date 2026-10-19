use dioxus::prelude::*;

pub mod context;

#[derive(Clone, Copy, PartialEq)]
pub struct CollisionSelection {
    selected_island: Signal<Option<String>>,
    selected_hotkey_unit: Signal<Option<String>>,
    selected_unit_position: Signal<Option<String>>,
}

impl CollisionSelection {
    pub fn new(
        selected_island: Signal<Option<String>>,
        selected_hotkey_unit: Signal<Option<String>>,
        selected_unit_position: Signal<Option<String>>,
    ) -> Self {
        Self {
            selected_island,
            selected_hotkey_unit,
            selected_unit_position,
        }
    }

    pub fn selected_island(&self) -> Signal<Option<String>> {
        self.selected_island
    }

    pub fn selected_hotkey_unit(&self) -> Signal<Option<String>> {
        self.selected_hotkey_unit
    }

    pub fn selected_unit_position(&self) -> Signal<Option<String>> {
        self.selected_unit_position
    }
}
