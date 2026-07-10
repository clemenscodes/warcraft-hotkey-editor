pub mod components;
mod props;
mod style;

use components::collision_card::CollisionCard;
use dioxus::prelude::*;
pub use props::CollisionListTrackProps;
use style::CLASS;
use tw_macro::assert_component;

/// The inner track that lays out the collision cards.
#[component]
pub fn CollisionListTrack(props: CollisionListTrackProps) -> Element {
    let cards = props.cards;
    rsx! {
        div {
            class: CLASS,
            for card in cards {
                CollisionCard { ..card }
            }
        }
    }
}

assert_component!(CollisionListTrack);
