use warcraft_keybinds::HotkeyToken;

/// The published `View` contract mirroring [`NormalHotkeyBadgeModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct NormalHotkeyBadgeView {
    pub letter: HotkeyToken,
}

impl ddd::View for NormalHotkeyBadgeView {}
