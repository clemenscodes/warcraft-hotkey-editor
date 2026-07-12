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

/// One move card: the reason badge, the fighting abilities (names over icons), and
/// the from -> to grids drawing where each ability lands. It owns only the grid root
/// and hands the move down to the panel that lays the card out.
#[component]
pub fn MoveCard(props: MoveCardModel) -> Element {
    let move_view = props.move_view;
    rsx! {
        div {
            class: CLASS,
            MovePanel { move_view }
        }
    }
}

assert_component!(MoveCard);
