pub mod components;
mod model;
mod presentation;
mod state;
mod view;

pub use state::GridTileState;
pub use view::GridTileView;

use components::empty_tile::EmptyTile;
use components::filled_tile::FilledTile;
use dioxus::prelude::*;
use model::GridTileModel;
use presentation::TileOccupancy;
use tw_macro::assert_component;

#[component]
pub fn GridTile(props: GridTileModel) -> Element {
    let GridTileModel {
        coordinate: _,
        icon,
        label,
        state,
        is_dragging_source,
        is_drag_over,
    } = props;
    let occupancy = TileOccupancy::from(state);
    match occupancy {
        TileOccupancy::Filled => rsx! {
            FilledTile {
                state,
                icon,
                label,
                is_dragging_source,
                is_drag_over,
            }
        },
        TileOccupancy::Empty => rsx! {
            EmptyTile {
                state,
                is_drag_over,
            }
        },
    }
}

assert_component!(GridTile);
