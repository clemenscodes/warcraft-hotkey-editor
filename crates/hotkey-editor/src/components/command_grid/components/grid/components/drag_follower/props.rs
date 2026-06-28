use dioxus::prelude::*;
use warcraft_api::Race;

use crate::model::grid::DragFollower;

use super::super::super::CommandGridProps;

#[derive(Props, Clone, PartialEq)]
pub struct DragFollowerOverlayProps {
    pub drag_follower: Signal<Option<DragFollower>>,
    #[props(default = Race::Neutral)]
    pub race: Race,
    /// Whether this grid owns the in-progress drag. The stylesheet is always
    /// emitted (so it is in `<head>` before any drag, avoiding a first-paint
    /// flicker), but the follower element only renders when visible.
    #[props(default)]
    pub visible: bool,
}

impl From<&CommandGridProps> for DragFollowerOverlayProps {
    /// The follower overlay needs only the grid's drag signal, its race, and
    /// whether the in-progress drag started in this grid. The last is true when
    /// the dragging slot reports this grid's id.
    fn from(props: &CommandGridProps) -> Self {
        let dragging_value = *props.dragging_slot.read();
        let visible = dragging_value
            .map(|detail| detail.grid_id() == props.grid_id)
            .unwrap_or(false);
        Self {
            drag_follower: props.drag_follower,
            race: props.race,
            visible,
        }
    }
}
