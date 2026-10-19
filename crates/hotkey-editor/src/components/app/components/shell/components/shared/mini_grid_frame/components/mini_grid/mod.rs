mod model;
mod view;

pub use view::MiniGridView;
mod style;

use crate::components::app::components::shell::components::shared::grid_tile::GridTile;
use dioxus::prelude::*;
use model::MiniGridModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MiniGrid(props: MiniGridModel) -> Element {
    let tiles = props.tiles;
    rsx! {
        div {
            class: CLASS,
            for tile in tiles {
                GridTile {
                    coordinate: tile.coordinate,
                    icon: tile.icon,
                    label: tile.label,
                    state: tile.state,
                    is_dragging_source: tile.is_dragging_source,
                    is_drag_over: tile.is_drag_over,
                }
            }
        }
    }
}

assert_component!(MiniGrid);
