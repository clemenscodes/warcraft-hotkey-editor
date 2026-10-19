pub mod components;
mod model;
mod presentation;
mod view;

pub use view::FilledTileView;
mod style;

use super::shared::drag_over_ring::DragOverRing;
use components::ability_fill::AbilityFill;
use components::command_fill::CommandFill;
use components::dragging_source_ghost::DraggingSourceGhost;
use components::selection_ring::SelectionRing;
use components::tile_icon::TileIcon;
use components::tile_label::TileLabel;
use dioxus::prelude::*;
use model::FilledTileModel;
use presentation::FilledTilePresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FilledTile(props: FilledTileModel) -> Element {
    let FilledTilePresentation {
        ability_active,
        command_active,
        selected,
        is_dragging_source,
        is_drag_over,
        icon_source,
        icon_alt,
        label_text,
    } = FilledTilePresentation::from(props);
    rsx! {
        div {
            class: CLASS,
            AbilityFill {
                active: ability_active,
            }
            CommandFill {
                active: command_active,
            }
            SelectionRing {
                selected,
            }
            TileIcon {
                src: icon_source,
                alt: icon_alt,
            }
            TileLabel {
                text: label_text,
            }
            DraggingSourceGhost {
                active: is_dragging_source,
            }
            DragOverRing {
                active: is_drag_over,
            }
        }
    }
}

assert_component!(FilledTile);
