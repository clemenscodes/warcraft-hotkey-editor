mod model;
mod view;

pub use view::MoveTransitionView;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::mobile_resolve::components::resolve_pager::components::resolve_pager_cards::components::shared::mini_grid::MiniGrid;
use dioxus::prelude::*;
use model::MoveTransitionModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MoveTransition(props: MoveTransitionModel) -> Element {
    let placements = props.placements;
    rsx! {
        div {
            class: CLASS,
            MiniGrid {
                placements,
            }
        }
    }
}

assert_component!(MoveTransition);
