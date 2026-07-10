pub mod components;
mod props;
mod view;

pub use view::CollisionListTrackView;
mod style;

use components::collision_card::{CollisionCard, CollisionCardData};
use dioxus::prelude::*;
use props::CollisionListTrackProps;
use style::CLASS;
use tw_macro::assert_component;

/// The inner track that lays out the collision cards.
#[component]
pub fn CollisionListTrack(props: CollisionListTrackProps) -> Element {
    let cards = props.cards;
    rsx! {
        div {
            class: CLASS,
            for CollisionCardData { is_selected, onclick, count, content } in cards {
                CollisionCard { is_selected, onclick, count, content }
            }
        }
    }
}

assert_component!(CollisionListTrack);
