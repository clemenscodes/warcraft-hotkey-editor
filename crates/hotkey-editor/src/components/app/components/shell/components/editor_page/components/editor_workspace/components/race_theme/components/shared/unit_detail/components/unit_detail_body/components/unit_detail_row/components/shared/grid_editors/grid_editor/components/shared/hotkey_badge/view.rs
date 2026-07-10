use super::state::HotkeyBadgeState;
use warcraft_keybinds::HotkeyToken;

/// The published `View` contract mirroring [`HotkeyBadgeProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HotkeyBadgeView {
    pub letter: HotkeyToken,
    pub state: HotkeyBadgeState,
}

impl ddd::View for HotkeyBadgeView {}
