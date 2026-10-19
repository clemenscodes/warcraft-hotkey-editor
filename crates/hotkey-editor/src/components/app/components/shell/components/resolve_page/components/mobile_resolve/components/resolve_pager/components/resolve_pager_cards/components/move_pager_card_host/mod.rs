pub mod components;
mod model;
mod view;

pub use view::MovePagerCardHostView;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::mobile_resolve::components::resolve_pager::components::resolve_pager_cards::components::move_pager_card_host::components::move_card::MoveCard;
use dioxus::prelude::*;
use model::MovePagerCardHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MovePagerCardHost(props: MovePagerCardHostModel) -> Element {
    let move_view = props.move_view;
    rsx! {
        div {
            class: CLASS,
            MoveCard {
                move_view,
            }
        }
    }
}

assert_component!(MovePagerCardHost);
