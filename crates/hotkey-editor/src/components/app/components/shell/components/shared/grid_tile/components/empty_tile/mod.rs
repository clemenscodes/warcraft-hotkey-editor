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
