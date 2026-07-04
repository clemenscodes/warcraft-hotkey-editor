use dioxus::prelude::*;

pub mod context;

/// The resolve page's selected move-category breadcrumb. Provided by the app shell
/// and read by the resolve page from context; the shell's URL sync reads it to write
/// the `?entry=` parameter, so the selection is shell-owned (the page must never
/// touch the router — it is shown in the gallery without one).
#[derive(Clone, Copy, PartialEq)]
pub struct ResolveSelection {
    pub selected_move_category: Signal<Option<String>>,
}
