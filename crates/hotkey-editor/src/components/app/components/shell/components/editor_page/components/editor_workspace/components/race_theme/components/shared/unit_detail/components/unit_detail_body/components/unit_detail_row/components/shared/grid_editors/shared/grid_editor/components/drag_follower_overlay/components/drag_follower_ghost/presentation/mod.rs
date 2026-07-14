use warcraft_keybinds::HotkeyToken;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::shared::grid_editor::components::shared::hotkey_badge::HotkeyBadgeState;
use crate::services::drag_state::DragFollower;
use super::state::GhostState;

#[derive(Clone, PartialEq)]
pub struct FollowerPresentation {
    pub(super) state: GhostState,
    pub(super) style: String,
    pub(super) badge_state: HotkeyBadgeState,
    pub(super) src: String,
    pub(super) alt: String,
    pub(super) letter: HotkeyToken,
}

impl From<&DragFollower> for FollowerPresentation {
    fn from(follower: &DragFollower) -> Self {
        let visual = follower.visual();
        let left = follower.left();
        let top = follower.top();
        let width = follower.tile_width();
        let height = follower.tile_height();
        let style = format!("left: {left}px; top: {top}px; width: {width}px; height: {height}px;",);
        let state = if visual.is_command_cell() {
            GhostState::Command
        } else {
            GhostState::Default
        };
        let badge_state = if visual.is_passive_command() {
            HotkeyBadgeState::Passive
        } else {
            HotkeyBadgeState::Normal
        };
        let src = visual.icon_source().to_string();
        let alt = visual.label_text().to_string();
        let letter = visual.displayed_letter();
        Self {
            state,
            style,
            badge_state,
            src,
            alt,
            letter,
        }
    }
}
