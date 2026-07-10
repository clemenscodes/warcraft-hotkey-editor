use warcraft_keybinds::HotkeyToken;

/// The published `View` contract mirroring [`ConflictHotkeyBadgeProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictHotkeyBadgeView {
    pub letter: HotkeyToken,
}

impl ddd::View for ConflictHotkeyBadgeView {}
