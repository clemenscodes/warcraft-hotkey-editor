pub mod components;
mod model;
mod view;

pub use view::MoveCardView;
mod style;

use components::move_panel::MovePanel;
use dioxus::prelude::*;
use model::MoveCardModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MoveCard(props: MoveCardModel) -> Element {
    let move_view = props.move_view;
    rsx! {
        div {
            class: CLASS,
            MovePanel {
                move_view,
            }
        }
    }
}

assert_component!(MoveCard);
