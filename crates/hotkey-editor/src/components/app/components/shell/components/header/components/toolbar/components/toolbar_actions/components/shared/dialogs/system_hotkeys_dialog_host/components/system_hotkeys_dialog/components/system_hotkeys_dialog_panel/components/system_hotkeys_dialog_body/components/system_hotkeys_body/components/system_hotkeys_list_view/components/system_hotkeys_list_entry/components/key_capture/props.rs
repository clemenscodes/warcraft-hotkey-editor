use dioxus::prelude::*;
use warcraft_keybinds::WarcraftObjectId;

/// A key chip for a list-view hotkey row: the section it binds. The editing section
/// comes from the dialog state context, and its resolved binding and conflicts come
/// from the CustomKeys query, so it needs neither the loaded keys nor a prebuilt
/// binding map.
#[derive(Props, Clone, PartialEq)]
pub struct KeyCaptureProps {
    pub section_id: WarcraftObjectId,
}
