pub mod components;
mod model;
mod view;

pub use view::CollisionCardButtonView;

use components::idle_collision_card_button::IdleCollisionCardButton;
use components::selected_collision_card_button::SelectedCollisionCardButton;
use dioxus::prelude::*;
use model::CollisionCardButtonModel;
use tw_macro::assert_component;

/// The collision card's selectable button. A pure dispatcher: from whether the card is
/// selected it renders `SelectedCollisionCardButton` xor `IdleCollisionCardButton`.
/// Each owns its `<button>` and its own look — the selected one wears the collision-gold
/// accent and publishes `--coordinate-color`; there is no `data-selected`, the look
/// follows the component.
#[component]
pub fn CollisionCardButton(props: CollisionCardButtonModel) -> Element {
    match props.is_selected {
        true => {
            let onclick = props.onclick;
            let count = props.count;
            let content = props.content;
            rsx! {
                SelectedCollisionCardButton { onclick, count, content }
            }
        }
        false => {
            let onclick = props.onclick;
            let count = props.count;
            let content = props.content;
            rsx! {
                IdleCollisionCardButton { onclick, count, content }
            }
        }
    }
}

assert_component!(CollisionCardButton);
