use super::view::PassiveHotkeyBadgeView;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

/// A passive-ability badge: it needs only the letter to draw.
#[derive(Props, Clone, PartialEq)]
pub struct PassiveHotkeyBadgeProps {
    pub letter: HotkeyToken,
}

impl From<&PassiveHotkeyBadgeView> for PassiveHotkeyBadgeProps {
    fn from(view: &PassiveHotkeyBadgeView) -> Self {
        let PassiveHotkeyBadgeView { letter } = view.clone();
        Self { letter }
    }
}

impl ddd::Props for PassiveHotkeyBadgeProps {
    type View = PassiveHotkeyBadgeView;
}
