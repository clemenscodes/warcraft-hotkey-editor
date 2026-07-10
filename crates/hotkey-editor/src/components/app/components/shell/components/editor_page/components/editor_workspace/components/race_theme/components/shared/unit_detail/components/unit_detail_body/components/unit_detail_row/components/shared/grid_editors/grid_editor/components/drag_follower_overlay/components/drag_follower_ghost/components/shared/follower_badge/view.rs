use warcraft_keybinds::HotkeyToken;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::shared::hotkey_badge::HotkeyBadgeState;

/// The published `View` contract mirroring [`FollowerBadgeProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FollowerBadgeView {
    pub letter: HotkeyToken,
    pub state: HotkeyBadgeState,
}

impl ddd::View for FollowerBadgeView {}
