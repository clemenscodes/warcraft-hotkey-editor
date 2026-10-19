use warcraft_keybinds::HotkeyToken;

#[derive(Clone, PartialEq)]
pub struct PassiveHotkeyBadgeView {
    pub letter: HotkeyToken,
}

impl ddd::Layered for PassiveHotkeyBadgeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PassiveHotkeyBadgeView {}
