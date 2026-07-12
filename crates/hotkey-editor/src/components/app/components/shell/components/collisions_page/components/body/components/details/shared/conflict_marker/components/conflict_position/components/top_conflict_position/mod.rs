mod model;
mod view;

pub use view::TopConflictPositionView;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::mini_grid::MiniGrid;
use dioxus::prelude::*;
use model::TopConflictPositionModel;
use style::CLASS;
use tw_macro::assert_component;

/// The colliding cell stacked over a multi-way ability row.
#[component]
pub fn TopConflictPosition(props: TopConflictPositionModel) -> Element {
    let coordinate = props.coordinate;
    rsx! {
        span {
            class: CLASS,
            MiniGrid { coordinate }
        }
    }
}

assert_component!(TopConflictPosition);
