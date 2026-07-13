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

/// The template preview's read-only command grid: the three-by-four square of
/// `TileFace` painters — the same tiles the editor draws, without any interaction.
/// A pure tile renderer that encodes the grid shape (shared with the editor and
/// mini grids via the same utility values) and draws whatever twelve tiles it is
/// handed.
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
