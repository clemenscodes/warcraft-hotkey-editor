mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::mini_grid::{MiniGrid, MiniGridProps};
use dioxus::prelude::*;
pub use props::InlineConflictPositionProps;
use style::CLASS;
use tw_macro::assert_component;

/// The colliding cell shown inline between a conflict's two abilities.
#[component]
pub fn InlineConflictPosition(props: InlineConflictPositionProps) -> Element {
    let coordinate = props.coordinate;
    let mini_grid = MiniGridProps { coordinate };
    rsx! {
        span {
            class: CLASS,
            MiniGrid { ..mini_grid }
        }
    }
}

assert_component!(InlineConflictPosition);
