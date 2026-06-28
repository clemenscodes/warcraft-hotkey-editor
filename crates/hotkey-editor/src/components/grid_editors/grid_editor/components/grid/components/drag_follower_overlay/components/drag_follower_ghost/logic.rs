use warcraft_keybinds::HotkeyToken;

use crate::components::grid_editors::grid_editor::HotkeyBadgeState;
use crate::model::grid::DragFollower;

/// The computed presentation for the follower ghost, derived from one
/// `DragFollower`. Field names match the attributes and child props they feed, so
/// the markup spreads them with RSX shorthand.
#[derive(Clone, PartialEq)]
pub struct FollowerPresentation {
    pub class: String,
    pub style: String,
    pub state: HotkeyBadgeState,
    pub src: String,
    pub alt: String,
    pub letter: HotkeyToken,
}

impl From<&DragFollower> for FollowerPresentation {
    /// Derives the follower's class, fixed position, badge, icon, and label from
    /// the dragged tile's captured visual.
    fn from(follower: &DragFollower) -> Self {
        let visual = follower.visual();
        let left = follower.left();
        let top = follower.top();
        let width = follower.tile_width();
        let height = follower.tile_height();
        let style = format!("left: {left}px; top: {top}px; width: {width}px; height: {height}px;");
        let mut class = String::from("drag-follower");
        if visual.is_command_cell() {
            class.push_str(" is-command");
        }
        let state = if visual.is_passive_command() {
            HotkeyBadgeState::Passive
        } else {
            HotkeyBadgeState::Normal
        };
        let src = visual.icon_source().to_string();
        let alt = visual.label_text().to_string();
        let letter = visual.displayed_letter();
        Self {
            class,
            style,
            state,
            src,
            alt,
            letter,
        }
    }
}
