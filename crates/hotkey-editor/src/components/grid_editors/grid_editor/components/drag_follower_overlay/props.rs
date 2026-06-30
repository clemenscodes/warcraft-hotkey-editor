use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_keybinds::GridBehavior;

use crate::components::grid_editors::grid_editor::GridEditorProps;
use crate::model::grid::DragFollower;

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

impl<B: GridBehavior> From<&GridEditorProps<B>> for DragFollowerOverlayProps {
    /// The follower overlay needs only the editor's drag signal, its race, and
    /// whether the in-progress drag started in this grid. The last is true when
    /// the dragging slot reports this grid's id.
    fn from(props: &GridEditorProps<B>) -> Self {
        let config = &props.config;
        let grid_id = config.heading;
        let dragging_value = *config.dragging_slot.read();
        let visible = dragging_value
            .map(|detail| detail.grid_id() == grid_id)
            .unwrap_or(false);
        let drag_follower = config.drag_follower;
        let race = config.race;
        Self {
            drag_follower,
            race,
            visible,
        }
    }
}
