mod props;
mod view;

pub use view::InlineConflictPositionView;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::mini_grid::MiniGrid;
use dioxus::prelude::*;
use props::InlineConflictPositionProps;
use style::CLASS;
use tw_macro::assert_component;

/// The colliding cell shown inline between a conflict's two abilities.
#[component]
pub fn InlineConflictPosition(props: InlineConflictPositionProps) -> Element {
    let coordinate = props.coordinate;
    rsx! {
        span {
            class: CLASS,
            MiniGrid { coordinate }
        }
    }
}

assert_component!(InlineConflictPosition);
