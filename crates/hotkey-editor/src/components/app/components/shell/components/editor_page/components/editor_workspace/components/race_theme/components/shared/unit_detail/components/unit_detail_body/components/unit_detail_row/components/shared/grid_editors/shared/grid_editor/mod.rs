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

/// The grid editor: a labeled, editable grid of a unit's command slots. It wraps
/// the presentational [`CaptionedEditorGrid`] verbatim and adds only behavior: it builds
/// the finished tiles with their drag handlers and renders the drag follower.
/// Generic over the [`GridBehavior`] that decides how moves cascade; the three
/// variant wrappers bind it. Pure RSX: it hands each child its data by named fields.
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
