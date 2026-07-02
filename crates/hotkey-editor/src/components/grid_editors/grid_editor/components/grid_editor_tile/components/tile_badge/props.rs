use super::super::super::GridEditorTileProps;
use super::super::hotkey_badge::{HotkeyBadgeProps, HotkeyBadgeState};
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

#[derive(Props, Clone, PartialEq)]
pub struct TileBadgeProps {
    pub letter: HotkeyToken,
    pub state: HotkeyBadgeState,
}

impl From<&GridEditorTileProps> for TileBadgeProps {
    fn from(props: &GridEditorTileProps) -> Self {
        let letter = props.hotkey;
        let state = props.badge_state;
        Self { letter, state }
    }
}

impl From<&TileBadgeProps> for HotkeyBadgeProps {
    fn from(props: &TileBadgeProps) -> Self {
        let letter = props.letter;
        let state = props.state;
        Self { letter, state }
    }
}
