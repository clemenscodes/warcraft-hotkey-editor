use super::super::super::HotkeyBadgeProps;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

/// A conflicting-binding badge: it needs only the letter to draw.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictHotkeyBadgeProps {
    pub letter: HotkeyToken,
}

impl From<&HotkeyBadgeProps> for ConflictHotkeyBadgeProps {
    fn from(props: &HotkeyBadgeProps) -> Self {
        let letter = props.letter;
        Self { letter }
    }
}
