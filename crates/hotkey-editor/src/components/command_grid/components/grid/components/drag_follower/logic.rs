use dioxus::prelude::*;
use warcraft_api::{Race, RaceLabels};

use crate::components::command_grid::HotkeyBadgeState;
use crate::model::grid::DragFollower;

/// Everything the overlay markup needs: the race attribute and, when this grid
/// owns the drag, the follower ghost's computed presentation. The component file
/// builds none of this; it destructures this and renders.
pub(super) struct OverlayRender {
    pub(super) race_attribute: &'static str,
    pub(super) follower: Option<FollowerPresentation>,
}

/// The computed presentation for the follower ghost: the strings and badge state
/// the markup renders, derived from one `DragFollower`.
pub(super) struct FollowerPresentation {
    pub(super) class_name: String,
    pub(super) position_style: String,
    pub(super) badge_state: HotkeyBadgeState,
    pub(super) icon_source: Option<String>,
    pub(super) label_text: String,
    pub(super) letter: Option<String>,
}

impl OverlayRender {
    /// Builds the overlay's render model: the race attribute, plus the follower's
    /// presentation when this grid owns the in-progress drag.
    pub(super) fn new(
        race: Race,
        visible: bool,
        drag_follower: Signal<Option<DragFollower>>,
    ) -> Self {
        let race_attribute = RaceLabels::data_attribute(race);
        let active = if visible {
            drag_follower.read().clone()
        } else {
            None
        };
        let follower = active.as_ref().map(FollowerPresentation::from);
        Self {
            race_attribute,
            follower,
        }
    }
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
        let position_style =
            format!("left: {left}px; top: {top}px; width: {width}px; height: {height}px;");
        let mut class_name = String::from("drag-follower");
        if visual.is_command_cell() {
            class_name.push_str(" is-command");
        }
        let badge_state = if visual.is_passive_command() {
            HotkeyBadgeState::Passive
        } else {
            HotkeyBadgeState::Normal
        };
        let icon_source = visual.icon_source().map(|icon| icon.to_string());
        let label_text = visual.label_text().to_string();
        let letter = visual.displayed_letter().map(|letter| letter.to_string());
        Self {
            class_name,
            position_style,
            badge_state,
            icon_source,
            label_text,
            letter,
        }
    }
}
