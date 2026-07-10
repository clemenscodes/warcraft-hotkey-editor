use super::view::TileBadgeView;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::shared::hotkey_badge::HotkeyBadgeState;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

#[derive(Props, Clone, PartialEq)]
pub struct TileBadgeProps {
    pub letter: HotkeyToken,
    pub state: HotkeyBadgeState,
}

impl From<&TileBadgeView> for TileBadgeProps {
    fn from(view: &TileBadgeView) -> Self {
        let TileBadgeView { letter, state } = view.clone();
        Self { letter, state }
    }
}

impl ddd::Props for TileBadgeProps {
    type View = TileBadgeView;
}
