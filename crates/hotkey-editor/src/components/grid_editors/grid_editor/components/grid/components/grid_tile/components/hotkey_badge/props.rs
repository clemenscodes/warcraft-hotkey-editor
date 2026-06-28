use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

use super::state::HotkeyBadgeState;

#[derive(Props, Clone, PartialEq)]
pub struct HotkeyBadgeProps {
    pub letter: HotkeyToken,
    #[props(default)]
    pub state: HotkeyBadgeState,
}
