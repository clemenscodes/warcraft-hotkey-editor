use dioxus::prelude::*;

pub mod context;

/// App-wide overlay open state: the visibility signals for the dialogs and panels
/// the toolbar and burger drawer toggle. Provided once at the app root and read
/// with `use_context`, so this state is never threaded as a prop through the
/// header. Each field is a `Signal`, so a reader subscribes only to the flag it
/// touches.
#[derive(Clone, Copy, PartialEq)]
pub struct OverlayState {
    preview_open: Signal<bool>,
    system_hotkeys_open: Signal<bool>,
    help_open: Signal<bool>,
    layout_dialog_open: Signal<bool>,
    templates_dialog_open: Signal<bool>,
}

impl OverlayState {
    pub fn new(
        preview_open: Signal<bool>,
        system_hotkeys_open: Signal<bool>,
        help_open: Signal<bool>,
        layout_dialog_open: Signal<bool>,
        templates_dialog_open: Signal<bool>,
    ) -> Self {
        Self {
            preview_open,
            system_hotkeys_open,
            help_open,
            layout_dialog_open,
            templates_dialog_open,
        }
    }

    pub fn preview_open(&self) -> Signal<bool> {
        self.preview_open
    }

    pub fn system_hotkeys_open(&self) -> Signal<bool> {
        self.system_hotkeys_open
    }

    pub fn help_open(&self) -> Signal<bool> {
        self.help_open
    }

    pub fn layout_dialog_open(&self) -> Signal<bool> {
        self.layout_dialog_open
    }

    pub fn templates_dialog_open(&self) -> Signal<bool> {
        self.templates_dialog_open
    }
}
