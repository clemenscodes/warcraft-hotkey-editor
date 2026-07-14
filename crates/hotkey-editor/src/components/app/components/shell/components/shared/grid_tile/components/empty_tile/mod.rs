pub mod components;
mod model;
mod presentation;
mod view;

pub use view::EmptyTileView;
mod style;

use super::shared::drag_over_ring::DragOverRing;
use components::blocked_drop_target_overlay::BlockedDropTargetOverlay;
use components::drop_target_overlay::DropTargetOverlay;
use components::highlight_overlay::HighlightOverlay;
use dioxus::prelude::*;
use model::EmptyTileModel;
use presentation::EmptyTilePresentation;
use style::CLASS;
use tw_macro::assert_component;

/// An empty command slot. Purely presentational: it draws its resting look and its race
/// accent, and — during a drag or a mini-grid highlight — mounts the matching overlay
/// child (drop-target / blocked / highlight, plus the `DragOverRing` under the cursor)
/// whose presence turns the tile's own border into that look. It knows nothing of
/// hotkeys, focus, or dragging; `GridEditorTile` layers all interaction on top.
#[component]
pub fn EmptyTile(props: EmptyTileModel) -> Element {
    let EmptyTilePresentation {
        drop_target_active,
        blocked_drop_target_active,
        highlight_active,
        is_drag_over,
    } = EmptyTilePresentation::from(props);
    rsx! {
        div {
            class: CLASS,
            DropTargetOverlay {
                active: drop_target_active,
            }
            BlockedDropTargetOverlay {
                active: blocked_drop_target_active,
            }
            HighlightOverlay {
                active: highlight_active,
            }
            DragOverRing {
                active: is_drag_over,
            }
        }
    }
}

assert_component!(EmptyTile);
