use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::shared::hotkey_badge::HotkeyBadgeState;
use warcraft_keybinds::HotkeyToken;

/// The published `View` contract mirroring [`TileBadgeProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TileBadgeView {
    pub letter: HotkeyToken,
    pub state: HotkeyBadgeState,
}

impl ddd::View for TileBadgeView {}
