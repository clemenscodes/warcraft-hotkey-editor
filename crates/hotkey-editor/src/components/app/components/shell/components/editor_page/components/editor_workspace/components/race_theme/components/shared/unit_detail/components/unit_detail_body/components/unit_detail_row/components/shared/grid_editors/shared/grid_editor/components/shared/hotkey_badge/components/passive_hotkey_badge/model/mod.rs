use super::view::PassiveHotkeyBadgeView;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

/// A passive-ability badge: it needs only the letter to draw.
#[derive(Props, Clone, PartialEq)]
pub struct PassiveHotkeyBadgeModel {
    pub letter: HotkeyToken,
}

impl From<&PassiveHotkeyBadgeView> for PassiveHotkeyBadgeModel {
    fn from(view: &PassiveHotkeyBadgeView) -> Self {
        let PassiveHotkeyBadgeView { letter } = view.clone();
        Self { letter }
    }
}

impl ddd::Model for PassiveHotkeyBadgeModel {
    type View = PassiveHotkeyBadgeView;
}
