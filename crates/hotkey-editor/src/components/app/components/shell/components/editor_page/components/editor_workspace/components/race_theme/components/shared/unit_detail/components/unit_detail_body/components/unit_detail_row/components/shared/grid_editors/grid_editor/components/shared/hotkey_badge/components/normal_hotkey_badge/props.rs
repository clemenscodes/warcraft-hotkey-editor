use super::super::super::HotkeyBadgeProps;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

/// An ordinary-binding badge: it needs only the letter to draw.
#[derive(Props, Clone, PartialEq)]
pub struct NormalHotkeyBadgeProps {
    pub letter: HotkeyToken,
}

impl From<&HotkeyBadgeProps> for NormalHotkeyBadgeProps {
    fn from(props: &HotkeyBadgeProps) -> Self {
        let letter = props.letter;
        Self { letter }
    }
}
