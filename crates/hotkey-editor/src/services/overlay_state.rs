use dioxus::prelude::*;

/// App-wide overlay open state: the visibility signals for the dialogs and panels
/// the toolbar and burger drawer toggle. Provided once at the app root and read
/// with `use_context`, so this state is never threaded as a prop through the
/// header. Each field is a `Signal`, so a reader subscribes only to the flag it
/// touches.
#[derive(Clone, Copy, PartialEq)]
pub struct OverlayState {
    pub preview_open: Signal<bool>,
    pub system_hotkeys_open: Signal<bool>,
    pub help_open: Signal<bool>,
    pub layout_dialog_open: Signal<bool>,
    pub templates_dialog_open: Signal<bool>,
}
