use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

use super::state::SystemKeyPickerKeyState;

/// One key on the system board, fully shaped by the picker hook: the domain key it
/// assigns, the label on its cap, its visual state, the tooltip text and its
/// placement/anchor, the wide flag for oversized caps, and the pick handler.
#[derive(Props, Clone, PartialEq)]
pub struct SystemKeyPickerKeyProps {
    pub label: &'static str,
    pub code: KeyCode,
    pub state: SystemKeyPickerKeyState,
    pub title: String,
    pub placement: &'static str,
    pub anchor: &'static str,
    pub wide: &'static str,
    pub on_pick: EventHandler<KeyCode>,
}
