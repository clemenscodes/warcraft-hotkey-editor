use super::state::HotkeyBadgeState;
use warcraft_keybinds::HotkeyToken;

#[derive(Clone, PartialEq)]
pub struct HotkeyBadgeView {
    pub letter: HotkeyToken,
    pub state: HotkeyBadgeState,
}

impl ddd::Layered for HotkeyBadgeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HotkeyBadgeView {}
