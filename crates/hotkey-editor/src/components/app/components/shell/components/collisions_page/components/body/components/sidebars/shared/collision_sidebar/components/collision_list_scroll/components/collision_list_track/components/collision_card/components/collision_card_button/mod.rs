pub mod components;
mod model;
mod view;

pub use view::CollisionCardButtonView;

use components::idle_collision_card_button::IdleCollisionCardButton;
use components::selected_collision_card_button::SelectedCollisionCardButton;
use dioxus::prelude::*;
use model::CollisionCardButtonModel;
use tw_macro::assert_component;

#[component]
pub fn CollisionCardButton(props: CollisionCardButtonModel) -> Element {
    match props.is_selected {
        true => {
            let onclick = props.onclick;
            let count = props.count;
            let content = props.content;
            rsx! {
                SelectedCollisionCardButton {
                    onclick,
                    count,
                    content,
                }
            }
        }
        false => {
            let onclick = props.onclick;
            let count = props.count;
            let content = props.content;
            rsx! {
                IdleCollisionCardButton {
                    onclick,
                    count,
                    content,
                }
            }
        }
    }
}

assert_component!(CollisionCardButton);
