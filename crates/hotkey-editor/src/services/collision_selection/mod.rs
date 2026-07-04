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
    pub selected_island: Signal<Option<String>>,
    pub selected_hotkey_unit: Signal<Option<String>>,
    pub selected_unit_position: Signal<Option<String>>,
}
