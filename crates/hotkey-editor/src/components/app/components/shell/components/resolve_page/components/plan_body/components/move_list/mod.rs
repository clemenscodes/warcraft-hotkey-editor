pub mod components;
mod model;
mod view;

pub use view::MoveListView;
mod style;

use components::move_card::MoveCard;
use dioxus::prelude::*;
use model::MoveListModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MoveList(props: MoveListModel) -> Element {
    let Some(section) = props.section else {
        return rsx! {};
    };
    let moves = section.moves().to_vec();
    rsx! {
        div {
            class: CLASS,
            for move_view in moves {
                MoveCard {
                    move_view,
                }
            }
        }
    }
}

assert_component!(MoveList);
