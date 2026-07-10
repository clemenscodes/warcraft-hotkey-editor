mod props;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::mini_grid::MiniGrid;
use dioxus::prelude::*;
pub use props::TransitionColumnProps;
use style::CLASS;
use tw_macro::assert_component;

/// One side (from or to) of the transition block: a single mini grid of placements.
#[component]
pub fn TransitionColumn(props: TransitionColumnProps) -> Element {
    let placements = props.placements;
    rsx! {
        div {
            class: CLASS,
            MiniGrid { placements }
        }
    }
}

assert_component!(TransitionColumn);
