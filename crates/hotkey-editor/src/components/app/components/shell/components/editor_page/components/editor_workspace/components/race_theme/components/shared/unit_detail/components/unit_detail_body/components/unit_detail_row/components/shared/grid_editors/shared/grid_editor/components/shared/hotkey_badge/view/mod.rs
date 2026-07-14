use super::state::HotkeyBadgeState;
use warcraft_keybinds::HotkeyToken;

#[derive(Clone, PartialEq)]
pub struct HotkeyBadgeView {
    pub letter: HotkeyToken,
    pub state: HotkeyBadgeState,
}

impl ddd::View for HotkeyBadgeView {}
