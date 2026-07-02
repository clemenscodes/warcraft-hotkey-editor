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
use style::CLASS;
use warcraft_keybinds::GridBehavior;
assert_component!(GridEditor);

/// The grid editor: a labeled, editable grid of a unit's command slots. It wraps
/// the presentational [`HeadedGrid`] verbatim and adds only behavior: it builds
/// the finished tiles with their drag handlers and renders the drag follower.
/// Generic over the [`GridBehavior`] that decides how moves cascade; the three
/// variant wrappers bind it. Pure RSX, every child's props is a `From`
/// conversion.
#[component]
pub(crate) fn GridEditor<B: GridBehavior>(props: GridEditorProps<B>) -> Element {
    rsx! {
        div { class: CLASS, "data-grid-id": props.config.heading,
            HeadedGrid { ..HeadedGridProps::<EditorTileKind>::from(&props) }
            DragFollowerOverlay { ..DragFollowerOverlayProps::from(&props) }
        }
    }
}
