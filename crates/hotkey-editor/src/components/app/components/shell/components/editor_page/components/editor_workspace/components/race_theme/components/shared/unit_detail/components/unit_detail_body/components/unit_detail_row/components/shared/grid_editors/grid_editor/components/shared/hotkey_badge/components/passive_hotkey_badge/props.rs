use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

/// A passive-ability badge: it needs only the letter to draw.
#[derive(Props, Clone, PartialEq)]
pub struct PassiveHotkeyBadgeProps {
    pub letter: HotkeyToken,
}
