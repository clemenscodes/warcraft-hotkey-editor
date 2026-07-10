use warcraft_keybinds::HotkeyToken;

/// The published `View` contract mirroring [`NormalHotkeyBadgeProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct NormalHotkeyBadgeView {
    pub letter: HotkeyToken,
}

impl ddd::View for NormalHotkeyBadgeView {}
