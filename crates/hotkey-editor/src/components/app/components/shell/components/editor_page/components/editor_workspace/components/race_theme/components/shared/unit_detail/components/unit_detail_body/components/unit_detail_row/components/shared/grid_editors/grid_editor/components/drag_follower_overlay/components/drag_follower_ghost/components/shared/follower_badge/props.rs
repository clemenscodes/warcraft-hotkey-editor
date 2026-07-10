use super::view::FollowerBadgeView;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::shared::hotkey_badge::HotkeyBadgeState;

#[derive(Props, Clone, PartialEq)]
pub struct FollowerBadgeProps {
    pub letter: HotkeyToken,
    pub state: HotkeyBadgeState,
}

impl From<&FollowerBadgeView> for FollowerBadgeProps {
    fn from(view: &FollowerBadgeView) -> Self {
        let FollowerBadgeView { letter, state } = view.clone();
        Self { letter, state }
    }
}

impl ddd::Props for FollowerBadgeProps {
    type View = FollowerBadgeView;
}
