use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker::KeyPickerCell;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

/// One pickable key: the cell it renders, whether a conflicting cell may still be
/// chosen, and the handler fired when it is picked. Everything the button shows
/// (label, title, disabled, special width, visual state) is derived from these in
/// `logic.rs`.
#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerKeyProps {
    pub cell: KeyPickerCell,
    pub allow_conflict_pick: bool,
    pub on_pick: EventHandler<HotkeyToken>,
}
