use warcraft_keybinds::HotkeyToken;

/// The published `View` contract mirroring [`PassiveHotkeyBadgeProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PassiveHotkeyBadgeView {
    pub letter: HotkeyToken,
}

impl ddd::View for PassiveHotkeyBadgeView {}
