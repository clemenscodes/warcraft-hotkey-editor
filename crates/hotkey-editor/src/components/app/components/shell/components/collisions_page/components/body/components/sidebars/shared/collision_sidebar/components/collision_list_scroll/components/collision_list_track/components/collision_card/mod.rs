pub mod components;
mod model;
mod view;

pub use view::CollisionCardView;
pub mod state;
mod style;

use components::collision_card_button::CollisionCardButton;
use dioxus::prelude::*;
use model::CollisionCardModel;
pub use state::{CollisionCardContent, CollisionCardData};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CollisionCard(props: CollisionCardModel) -> Element {
    let is_selected = props.is_selected;
    let onclick = props.onclick;
    let count = props.count;
    let content = props.content;
    rsx! {
        div {
            class: CLASS,
            CollisionCardButton {
                is_selected,
                onclick,
                count,
                content,
            }
        }
    }
}

assert_component!(CollisionCard);
