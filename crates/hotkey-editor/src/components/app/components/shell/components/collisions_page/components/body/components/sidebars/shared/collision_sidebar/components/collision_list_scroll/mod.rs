pub mod components;
mod props;
mod style;

use components::collision_list_track::CollisionListTrack;
use dioxus::prelude::*;
use props::CollisionListScrollProps;
use style::CLASS;
use tw_macro::assert_component;

/// The scrolling region of a collision sidebar: a vertical list, or the swipe
/// carousel on small screens. Lays the cards out through the collision track.
#[component]
pub fn CollisionListScroll(props: CollisionListScrollProps) -> Element {
    let cards = props.cards;
    rsx! {
        div {
            class: CLASS,
            CollisionListTrack { cards }
        }
    }
}

assert_component!(CollisionListScroll);
