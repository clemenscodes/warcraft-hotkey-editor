use super::view::NormalHotkeyBadgeView;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

/// An ordinary-binding badge: it needs only the letter to draw.
#[derive(Props, Clone, PartialEq)]
pub struct NormalHotkeyBadgeProps {
    pub letter: HotkeyToken,
}

impl From<&NormalHotkeyBadgeView> for NormalHotkeyBadgeProps {
    fn from(view: &NormalHotkeyBadgeView) -> Self {
        let NormalHotkeyBadgeView { letter } = view.clone();
        Self { letter }
    }
}

impl ddd::Props for NormalHotkeyBadgeProps {
    type View = NormalHotkeyBadgeView;
}
