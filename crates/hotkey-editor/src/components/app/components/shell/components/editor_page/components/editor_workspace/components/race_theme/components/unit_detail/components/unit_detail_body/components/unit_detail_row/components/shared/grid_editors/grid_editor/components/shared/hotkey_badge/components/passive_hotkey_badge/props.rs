use super::super::super::HotkeyBadgeProps;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

/// A passive-ability badge: it needs only the letter to draw.
#[derive(Props, Clone, PartialEq)]
pub struct PassiveHotkeyBadgeProps {
    pub letter: HotkeyToken,
}

impl From<&HotkeyBadgeProps> for PassiveHotkeyBadgeProps {
    fn from(props: &HotkeyBadgeProps) -> Self {
        let letter = props.letter;
        Self { letter }
    }
}
