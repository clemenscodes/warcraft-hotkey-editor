use super::state::KeyPickerCell;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

/// What the key picker needs: the title the shell shows, the board of keys to
/// offer, the open flag that mounts it, and the handlers for a pick and a close.
/// `allow_conflict_pick` lets a conflicting key stay pickable (the layout editor
/// swaps the two), which the spell picker leaves off so a clash is flagged but
/// cannot be chosen.
#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerProps {
    #[props(into)]
    pub title: String,
    pub rows: Vec<Vec<KeyPickerCell>>,
    pub open: bool,
    #[props(default = false)]
    pub allow_conflict_pick: bool,
    pub on_pick: EventHandler<HotkeyToken>,
    pub on_close: EventHandler<()>,
}
