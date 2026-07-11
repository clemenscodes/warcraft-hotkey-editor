use super::view::ConflictHotkeyBadgeView;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

/// A conflicting-binding badge: it needs only the letter to draw.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictHotkeyBadgeModel {
    pub letter: HotkeyToken,
}

impl From<&ConflictHotkeyBadgeView> for ConflictHotkeyBadgeModel {
    fn from(view: &ConflictHotkeyBadgeView) -> Self {
        let ConflictHotkeyBadgeView { letter } = view.clone();
        Self { letter }
    }
}

impl ddd::Model for ConflictHotkeyBadgeModel {
    type View = ConflictHotkeyBadgeView;
}
