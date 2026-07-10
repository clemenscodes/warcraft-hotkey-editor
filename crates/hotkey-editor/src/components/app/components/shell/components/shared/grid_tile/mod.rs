pub mod components;
mod logic;
mod props;
mod state;
mod view;

pub use state::GridTileState;
pub use view::GridTileView;

use components::empty_tile::EmptyTile;
use components::filled_tile::FilledTile;
use dioxus::prelude::*;
use logic::TileOccupancy;
use props::GridTileProps;
use tw_macro::assert_component;

/// A command-grid slot. A pure dispatcher: from the slot's state it renders the
/// occupied tile (`FilledTile`) or the empty one (`EmptyTile`). The occupancy decision
/// lives in `TileOccupancy`, so the body only decides and renders — it hands each child
/// its data as named fields and never builds a child's props.
#[component]
pub fn GridTile(props: GridTileProps) -> Element {
    let GridTileProps {
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
