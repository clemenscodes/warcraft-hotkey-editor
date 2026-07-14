pub mod components;
mod model;
mod view;

pub use view::CollisionListTrackView;
mod style;

use components::collision_card::{CollisionCard, CollisionCardData};
use dioxus::prelude::*;
use model::CollisionListTrackModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CollisionListTrack(props: CollisionListTrackModel) -> Element {
    let cards = props.cards;
    rsx! {
        div {
            class: CLASS,
            for CollisionCardData { is_selected, onclick, count, content } in cards {
                CollisionCard {
                    is_selected,
                    onclick,
                    count,
                    content,
                }
            }
        }
    }
}

assert_component!(CollisionListTrack);
