use warcraft_keybinds::HotkeyToken;

#[derive(Clone, PartialEq)]
pub struct ConflictHotkeyBadgeView {
    pub letter: HotkeyToken,
}

impl ddd::View for ConflictHotkeyBadgeView {}
