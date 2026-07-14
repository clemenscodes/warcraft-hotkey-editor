use warcraft_keybinds::HotkeyToken;

#[derive(Clone, PartialEq)]
pub struct PassiveHotkeyBadgeView {
    pub letter: HotkeyToken,
}

impl ddd::View for PassiveHotkeyBadgeView {}
