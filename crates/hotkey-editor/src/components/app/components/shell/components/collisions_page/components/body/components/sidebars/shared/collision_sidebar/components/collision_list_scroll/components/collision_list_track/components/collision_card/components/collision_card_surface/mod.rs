pub mod components;
mod logic;
mod props;

use components::idle_collision_card_surface::{
    IdleCollisionCardSurface, IdleCollisionCardSurfaceProps,
};
use components::selected_collision_card_surface::{
    SelectedCollisionCardSurface, SelectedCollisionCardSurfaceProps,
};
use dioxus::prelude::*;
pub use props::CollisionCardSurfaceProps;
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
            let surface = SelectedCollisionCardSurfaceProps::from(&props);
            rsx! {
                SelectedCollisionCardSurface { ..surface }
            }
        }
        false => {
            let surface = IdleCollisionCardSurfaceProps::from(&props);
            rsx! {
                IdleCollisionCardSurface { ..surface }
            }
        }
    }
}

assert_component!(CollisionCardSurface);
