use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

/// An ordinary-binding badge: it needs only the letter to draw.
#[derive(Props, Clone, PartialEq)]
pub struct NormalHotkeyBadgeProps {
    pub letter: HotkeyToken,
}
