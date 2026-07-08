pub mod components;
mod props;
mod style;

use components::collision_list_track::{CollisionListTrack, CollisionListTrackProps};
use dioxus::prelude::*;
pub use props::CollisionListScrollProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(CollisionListScroll);

/// The scrolling region of a collision sidebar: a vertical list, or the swipe
/// carousel on small screens. Lays the cards out through the collision track.
#[component]
pub fn CollisionListScroll(props: CollisionListScrollProps) -> Element {
    let cards = props.cards;
    let track = CollisionListTrackProps { cards };
    rsx! {
        div {
            class: CLASS,
            CollisionListTrack { ..track }
        }
    }
}
