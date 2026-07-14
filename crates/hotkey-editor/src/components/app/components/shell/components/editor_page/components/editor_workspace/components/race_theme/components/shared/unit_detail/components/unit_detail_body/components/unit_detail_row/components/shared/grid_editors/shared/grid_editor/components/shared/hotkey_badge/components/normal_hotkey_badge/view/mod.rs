use warcraft_keybinds::HotkeyToken;

#[derive(Clone, PartialEq)]
pub struct NormalHotkeyBadgeView {
    pub letter: HotkeyToken,
}

impl ddd::View for NormalHotkeyBadgeView {}
