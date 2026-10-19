mod model;
mod presentation;
mod view;

pub use view::PreviewGridView;
mod style;

use crate::components::app::components::shell::components::shared::tile_face::TileFace;
use dioxus::prelude::*;
use model::PreviewGridModel;
use presentation::use_preview_grid;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PreviewGrid(props: PreviewGridModel) -> Element {
    let tiles = use_preview_grid(&props);
    rsx! {
        div {
            class: CLASS,
            for tile in tiles {
                TileFace {
                    coordinate: tile.coordinate,
                    icon: tile.icon,
                    label: tile.label,
                    hotkey: tile.hotkey,
                    badge_state: tile.badge_state,
                    state: tile.state,
                }
            }
        }
    }
}

assert_component!(PreviewGrid);
