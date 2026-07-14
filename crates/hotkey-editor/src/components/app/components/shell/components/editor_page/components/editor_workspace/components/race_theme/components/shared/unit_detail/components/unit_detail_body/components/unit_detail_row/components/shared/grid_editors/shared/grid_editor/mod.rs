pub mod components;
mod model;
mod presentation;
mod style;
mod view;

pub use view::GridEditorView;

use model::GridEditorModel;

use components::captioned_editor_grid::CaptionedEditorGrid;
use components::drag_follower_overlay::DragFollowerOverlay;
use dioxus::prelude::*;
use presentation::{GridEditorPresentation, use_grid_editor};
use style::CLASS;
use tw_macro::assert_component;
use warcraft_keybinds::GridBehavior;

#[component]
pub(crate) fn GridEditor<B: GridBehavior>(props: GridEditorModel<B>) -> Element {
    let GridEditorPresentation {
        tiles,
        visible,
        heading,
        drag_follower,
    } = use_grid_editor(&props);
    rsx! {
        div {
            class: CLASS,
            CaptionedEditorGrid {
                heading,
                tiles,
            }
            DragFollowerOverlay {
                drag_follower,
                visible,
            }
        }
    }
}

assert_component!(GridEditor);
