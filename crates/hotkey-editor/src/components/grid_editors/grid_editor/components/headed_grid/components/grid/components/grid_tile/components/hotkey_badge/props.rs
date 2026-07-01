use super::state::HotkeyBadgeState;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

#[derive(Props, Clone, PartialEq)]
pub struct HotkeyBadgeProps {
    pub letter: HotkeyToken,
    #[props(default)]
    pub state: HotkeyBadgeState,
}
