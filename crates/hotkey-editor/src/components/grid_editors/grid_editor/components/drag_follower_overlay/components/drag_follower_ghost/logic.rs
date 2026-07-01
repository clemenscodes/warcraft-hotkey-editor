use warcraft_keybinds::HotkeyToken;
use crate::components::grid_editors::grid_editor::components::headed_grid::components::grid::components::grid_tile::components::hotkey_badge::HotkeyBadgeState;
use crate::model::grid::DragFollower;
use super::state::GhostState;

/// The computed presentation for the follower ghost, derived from one
/// `DragFollower`. Field names match the attributes and child props they feed, so
/// the markup spreads them with RSX shorthand.
#[derive(Clone, PartialEq)]
pub struct FollowerPresentation {
    pub state: GhostState,
    pub style: String,
    pub badge_state: HotkeyBadgeState,
    pub src: String,
    pub alt: String,
    pub letter: HotkeyToken,
}

impl From<&DragFollower> for FollowerPresentation {
    /// Derives the follower's state, fixed position, badge, icon, and label from
    /// the dragged tile's captured visual.
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
