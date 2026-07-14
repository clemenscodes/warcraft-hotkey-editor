use super::view::NormalHotkeyBadgeView;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

#[derive(Props, Clone, PartialEq)]
pub struct NormalHotkeyBadgeModel {
    pub letter: HotkeyToken,
}

impl From<&NormalHotkeyBadgeView> for NormalHotkeyBadgeModel {
    fn from(view: &NormalHotkeyBadgeView) -> Self {
        let NormalHotkeyBadgeView { letter } = view.clone();
        Self { letter }
    }
}

impl ddd::Model for NormalHotkeyBadgeModel {
    type View = NormalHotkeyBadgeView;
}
