use warcraft_api::{Race, RaceLabels};

use super::state::GridTileState;

/// The computed presentation for one tile: every string and attribute the markup
/// needs, derived from the tile's props. The component file builds none of this
/// itself; it destructures this and renders.
pub(super) struct GridTilePresentation {
    pub(super) class_name: String,
    pub(super) tabindex: &'static str,
    pub(super) draggable_attribute: &'static str,
    pub(super) race_attribute: &'static str,
}

impl GridTilePresentation {
    /// Derives the tile's class list, focus, draggable, and race attributes from
    /// its state flags.
    pub(super) fn new(
        state: GridTileState,
        is_dragging_source: bool,
        is_drag_over: bool,
        is_focusable: bool,
        draggable: bool,
        race: Race,
    ) -> Self {
        let mut class_name = String::from("grid-tile");
        let base = state.base_class();
        if !base.is_empty() {
            class_name.push(' ');
            class_name.push_str(base);
        }
        if is_dragging_source {
            class_name.push_str(" dragging-source");
        }
        if is_drag_over {
            class_name.push_str(" drag-over");
        }
        let tabindex = if is_focusable { "0" } else { "-1" };
        let draggable_attribute = if draggable { "true" } else { "false" };
        let race_attribute = RaceLabels::data_attribute(race);
        Self {
            class_name,
            tabindex,
            draggable_attribute,
            race_attribute,
        }
    }
}
