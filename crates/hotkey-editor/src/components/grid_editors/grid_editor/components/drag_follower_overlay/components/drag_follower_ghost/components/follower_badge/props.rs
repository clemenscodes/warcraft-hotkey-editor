use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

use crate::components::grid_editors::grid_editor::components::headed_grid::components::grid::components::grid_tile::components::hotkey_badge::{
    HotkeyBadgeProps, HotkeyBadgeState,
};

use super::super::super::logic::FollowerPresentation;

#[derive(Props, Clone, PartialEq)]
pub struct FollowerBadgeProps {
    pub letter: HotkeyToken,
    pub state: HotkeyBadgeState,
}

impl From<&FollowerPresentation> for FollowerBadgeProps {
    fn from(presentation: &FollowerPresentation) -> Self {
        let letter = presentation.letter;
        let state = presentation.badge_state;
        Self { letter, state }
    }
}

impl From<&FollowerBadgeProps> for HotkeyBadgeProps {
    fn from(props: &FollowerBadgeProps) -> Self {
        let letter = props.letter;
        let state = props.state;
        Self { letter, state }
    }
}
