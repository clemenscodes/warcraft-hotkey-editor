pub mod components;
mod model;
mod style;
mod view;

pub use view::TileFaceView;

use crate::components::app::components::shell::components::shared::grid_tile::GridTile;
use components::tile_badge::TileBadge;
use dioxus::prelude::*;
use model::TileFaceModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn TileFace(props: TileFaceModel) -> Element {
    let coordinate = props.coordinate;
    let icon = props.icon.clone();
    let label = props.label.clone();
    let state = props.state;
    let is_dragging_source = props.is_dragging_source;
    let is_drag_over = props.is_drag_over;
    let letter = props.hotkey;
    let badge_state = props.badge_state;
    rsx! {
        div {
            class: CLASS,
            GridTile {
                coordinate,
                icon,
                label,
                state,
                is_dragging_source,
                is_drag_over,
            }
            TileBadge {
                letter,
                state: badge_state,
            }
        }
    }
}

assert_component!(TileFace);
