use dioxus::prelude::*;

pub mod context;

/// Presentation state that lets a picked search result close the search dialog.
///
/// The dialog owns its own open flag and hands this dismiss down through
/// context, so the shared unit card closes the overlay on a pick without ever
/// naming the flag. Outside the dialog the context is absent, so the same card
/// renders unchanged in the desktop unit list.
#[derive(Clone, Copy, PartialEq)]
pub struct SearchDialogDismiss {
    on_open_change: Callback<bool>,
}

impl SearchDialogDismiss {
    pub fn new(on_open_change: Callback<bool>) -> Self {
        Self { on_open_change }
    }

    pub fn dismiss(&self) {
        self.on_open_change.call(false);
    }
}
