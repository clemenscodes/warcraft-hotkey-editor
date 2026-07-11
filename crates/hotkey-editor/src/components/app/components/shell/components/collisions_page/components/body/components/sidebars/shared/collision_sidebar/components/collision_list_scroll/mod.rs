pub mod components;
mod model;
mod view;

pub use view::CollisionListScrollView;
mod style;

use components::collision_list_track::CollisionListTrack;
use dioxus::prelude::*;
use model::CollisionListScrollModel;
use style::CLASS;
use tw_macro::assert_component;

/// The scrolling region of a collision sidebar: a vertical list, or the swipe
/// carousel on small screens. Lays the cards out through the collision track.
#[component]
pub fn CollisionListScroll(props: CollisionListScrollModel) -> Element {
    let cards = props.cards;
    rsx! {
        div {
            class: CLASS,
            CollisionListTrack { cards }
        }
    }
}

assert_component!(CollisionListScroll);
