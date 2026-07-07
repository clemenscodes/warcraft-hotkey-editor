use dioxus::prelude::*;

pub mod context;

/// The collisions page's selected entry, one signal per kind so each tab keeps its
/// own last selection. Provided by the app shell and read by the collisions page
/// from context; the shell's URL sync reads the active kind's signal to write the
/// `?entry=` parameter, which is why the selection is shell-owned rather than
/// page-local (the page must never touch the router — it is shown in the gallery
/// without one).
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
