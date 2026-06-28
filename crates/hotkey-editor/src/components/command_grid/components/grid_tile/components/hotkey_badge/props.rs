use dioxus::prelude::*;

use super::state::HotkeyBadgeState;

#[derive(Props, Clone, PartialEq)]
pub struct HotkeyBadgeProps {
    pub letter: String,
    #[props(default)]
    pub state: HotkeyBadgeState,
}
