use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

use super::super::super::GridTileProps;
use super::super::{HotkeyBadgeProps, HotkeyBadgeState};

#[derive(Props, Clone, PartialEq)]
pub struct TileBadgeProps {
    pub letter: HotkeyToken,
    pub state: HotkeyBadgeState,
}

impl From<&GridTileProps> for TileBadgeProps {
    fn from(props: &GridTileProps) -> Self {
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
