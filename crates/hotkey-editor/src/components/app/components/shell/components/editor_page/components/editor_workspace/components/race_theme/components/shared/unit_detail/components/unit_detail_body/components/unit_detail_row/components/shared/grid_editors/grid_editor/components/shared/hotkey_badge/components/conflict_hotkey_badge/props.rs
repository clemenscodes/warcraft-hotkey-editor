use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

/// A conflicting-binding badge: it needs only the letter to draw.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictHotkeyBadgeProps {
    pub letter: HotkeyToken,
}
