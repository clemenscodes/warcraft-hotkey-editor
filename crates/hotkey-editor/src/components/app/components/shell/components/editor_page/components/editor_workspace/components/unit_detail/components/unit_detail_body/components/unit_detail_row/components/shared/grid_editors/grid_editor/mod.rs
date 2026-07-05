pub mod components;
mod logic;
mod props;
mod style;

use crate::assert_component;
use components::drag_follower_overlay::{DragFollowerOverlay, DragFollowerOverlayProps};
use components::grid_editor_tile::EditorTileKind;
use components::headed_grid::{HeadedGrid, HeadedGridProps};
use dioxus::prelude::*;
pub use props::{GridEditorConfig, GridEditorProps};
use std::rc::Rc;
use style::CLASS;
use warcraft_keybinds::{CommandGridRenderInput, GridBehavior, GridSlotId, RenderedTile};
assert_component!(GridEditor);

/// The grid editor: a labeled, editable grid of a unit's command slots. It wraps
/// the presentational [`HeadedGrid`] verbatim and adds only behavior: it builds
/// the finished tiles with their drag handlers and renders the drag follower.
/// Generic over the [`GridBehavior`] that decides how moves cascade; the three
/// variant wrappers bind it. Pure RSX, every child's props is a `From`
/// conversion.
///
/// The rendered tiles are computed in a `use_memo` here, in this component's own
/// reactive scope, rather than inside `HeadedGridProps::from_parts`. That is what
/// makes the memoization work: only this closure subscribes to `loaded_keys` and
/// the other grid-state signals, so `GridEditor` itself re-renders only when its
/// own memoized `Vec<RenderedTile>` compares unequal — not whenever any sibling
/// grid's slots change.
#[component]
pub(crate) fn GridEditor<B: GridBehavior>(props: GridEditorProps<B>) -> Element {
    let config = &props.config;
    let loaded_keys = config.loaded_keys;
    let tier_overrides = config.tier_overrides;
    let grid_layout = config.grid_layout;
    let selected_slot = config.selected_slot;
    let selected_from_research = config.selected_from_research;
    let slot_ids = config.slot_ids.clone();
    let restrict_draggable_to: Rc<[GridSlotId]> = Rc::from(config.restrict_draggable_to.as_slice());
    let behavior = props.behavior.clone();
    let rendered_tiles = use_memo(move || {
        let read_guard = loaded_keys.read();
        let Some(file) = read_guard.as_ref() else {
            return Vec::<RenderedTile>::new();
        };
        let tier_guard = tier_overrides.read();
        let layout_snapshot = *grid_layout.read();
        let selected_snapshot = *selected_slot.read();
        let selected_research_snapshot = *selected_from_research.read();
        let input = CommandGridRenderInput {
            slots: &slot_ids,
            layout: layout_snapshot,
            selected: selected_snapshot,
            selected_is_research: selected_research_snapshot,
            tier_overrides: &tier_guard,
            restrict_draggable_to: &restrict_draggable_to,
        };
        file.rendered_command_grid(&behavior, &input)
    });
    rsx! {
        div { class: CLASS, "data-grid-id": props.config.heading,
            HeadedGrid { ..HeadedGridProps::<EditorTileKind>::from_parts(&props, rendered_tiles.read().clone()) }
            DragFollowerOverlay { ..DragFollowerOverlayProps::from(&props) }
        }
    }
}
