mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::mini_grid::{MiniGrid, MiniGridProps};
use dioxus::prelude::*;
pub use props::TopConflictPositionProps;
use style::CLASS;
use tw_macro::assert_component;

/// The colliding cell stacked over a multi-way ability row.
#[component]
pub fn TopConflictPosition(props: TopConflictPositionProps) -> Element {
    let coordinate = props.coordinate;
    let mini_grid = MiniGridProps { coordinate };
    rsx! {
        span {
            class: CLASS,
            MiniGrid { ..mini_grid }
        }
    }
}

assert_component!(TopConflictPosition);
