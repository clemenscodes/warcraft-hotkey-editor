mod drag_state;
mod handlers;
mod mechanics;
mod render;

use super::components::captioned_editor_grid::components::editor_grid::components::grid_editor_tile::EditorTile;
use super::model::GridEditorModel;
use crate::services::drag_state::DragFollower;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_keybinds::{
    COMMAND_GRID_TILE_COUNT, CommandGridRenderInput, GridBehavior, GridSlotId, RenderedTile,
};

#[derive(Clone, PartialEq)]
pub(crate) struct GridEditorPresentation {
    pub(super) tiles: [EditorTile; COMMAND_GRID_TILE_COUNT],
    pub(super) visible: bool,
    pub(super) heading: &'static str,
    pub(super) drag_follower: Signal<Option<DragFollower>>,
}

pub(crate) fn use_grid_editor<B: GridBehavior>(
    props: &GridEditorModel<B>,
) -> GridEditorPresentation {
    let config = &props.config;
    let loaded_keys = config.loaded_keys;
    let tier_overrides = config.tier_overrides;
    let grid_layout = config.grid_layout;
    let selected_slot = config.selected_slot;
    let selected_from_research = config.selected_from_research;
    let slot_ids = config.slot_ids.clone();
    let restrict_draggable_to: Rc<[GridSlotId]> = Rc::from(config.restrict_draggable_to.as_slice());
    let behavior = props.behavior.clone();
    let rendered_tiles = use_memo(use_reactive!(|slot_ids| {
        let read_guard = loaded_keys.read();
        let Some(file) = read_guard.as_ref() else {
            return Vec::<RenderedTile>::new();
        };
        let tier_guard = tier_overrides.read();
        let layout_snapshot = *grid_layout.read();
        let selected_snapshot = *selected_slot.read();
        let selected_research_snapshot = *selected_from_research.read();
        let input = CommandGridRenderInput::new(
            &slot_ids,
            layout_snapshot,
            selected_snapshot,
            selected_research_snapshot,
            &tier_guard,
            &restrict_draggable_to,
        );
        file.rendered_command_grid(&behavior, &input)
    }));
    let heading = config.heading;
    let drag_follower = config.drag_follower;
    let dragging_value = *config.dragging_slot.read();
    let visible = dragging_value
        .map(|detail| detail.grid_id() == heading)
        .unwrap_or(false);
    let rendered = rendered_tiles.read().clone();
    let tiles = EditorTile::grid(props, rendered);
    GridEditorPresentation {
        tiles,
        visible,
        heading,
        drag_follower,
    }
}
