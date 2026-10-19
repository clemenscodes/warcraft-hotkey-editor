use warcraft_keybinds::HotkeyToken;

#[derive(Clone, PartialEq)]
pub struct NormalHotkeyBadgeView {
    pub letter: HotkeyToken,
}

impl ddd::Layered for NormalHotkeyBadgeView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for NormalHotkeyBadgeView {}
