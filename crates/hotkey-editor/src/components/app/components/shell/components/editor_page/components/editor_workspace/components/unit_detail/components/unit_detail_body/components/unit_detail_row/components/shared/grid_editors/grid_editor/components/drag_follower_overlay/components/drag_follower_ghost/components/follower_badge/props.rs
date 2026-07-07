use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::shared::hotkey_badge::{
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
