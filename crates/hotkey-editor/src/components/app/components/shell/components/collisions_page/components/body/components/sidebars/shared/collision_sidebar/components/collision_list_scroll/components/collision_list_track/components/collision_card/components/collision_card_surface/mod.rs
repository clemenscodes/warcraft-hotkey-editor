pub mod components;
mod props;

use components::idle_collision_card_surface::IdleCollisionCardSurface;
use components::selected_collision_card_surface::SelectedCollisionCardSurface;
use dioxus::prelude::*;
use props::CollisionCardSurfaceProps;
use tw_macro::assert_component;

/// The collision card's selectable button. A pure dispatcher: from whether the card is
/// selected it renders `SelectedCollisionCardSurface` xor `IdleCollisionCardSurface`.
/// Each owns its `<button>` and its own look — the selected one wears the collision-gold
/// accent and publishes `--coordinate-color`; there is no `data-selected`, the look
/// follows the component.
#[component]
pub fn CollisionCardSurface(props: CollisionCardSurfaceProps) -> Element {
    match props.is_selected {
        true => {
            let onclick = props.onclick;
            let count = props.count;
            let content = props.content;
            rsx! {
                SelectedCollisionCardSurface { onclick, count, content }
            }
        }
        false => {
            let onclick = props.onclick;
            let count = props.count;
            let content = props.content;
            rsx! {
                IdleCollisionCardSurface { onclick, count, content }
            }
        }
    }
}

assert_component!(CollisionCardSurface);
