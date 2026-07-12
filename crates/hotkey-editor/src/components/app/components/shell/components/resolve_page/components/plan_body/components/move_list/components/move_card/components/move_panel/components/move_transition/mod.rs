pub mod components;
mod model;
mod view;

pub use view::MoveTransitionView;
mod style;

use components::move_arrow::MoveArrow;
use components::transition_column::TransitionColumn;
use dioxus::prelude::*;
use model::MoveTransitionModel;
use style::CLASS;
use tw_macro::assert_component;

/// The from → to transition block: the "before" grid, the centred arrow, and the
/// "after" grid.
#[component]
pub fn MoveTransition(props: MoveTransitionModel) -> Element {
    let from_placements = props.from_placements;
    let to_placements = props.to_placements;
    rsx! {
        div {
            class: CLASS,
            TransitionColumn { placements: from_placements }
            MoveArrow {}
            TransitionColumn { placements: to_placements }
        }
    }
}

assert_component!(MoveTransition);
