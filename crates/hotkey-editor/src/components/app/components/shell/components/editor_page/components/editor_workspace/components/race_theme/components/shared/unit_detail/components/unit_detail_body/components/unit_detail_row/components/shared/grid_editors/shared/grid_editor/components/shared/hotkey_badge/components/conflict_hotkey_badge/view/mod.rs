use warcraft_keybinds::HotkeyToken;

#[derive(Clone, PartialEq)]
pub struct ConflictHotkeyBadgeView {
    pub letter: HotkeyToken,
}

impl ddd::Layered for ConflictHotkeyBadgeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ConflictHotkeyBadgeView {}
