pub mod components;
mod model;
mod view;

pub use view::IslandPagerCardHostView;
mod style;

use components::island_pager_card::IslandPagerCard;
use dioxus::prelude::*;
use model::IslandPagerCardHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn IslandPagerCardHost(props: IslandPagerCardHostModel) -> Element {
    let island = props.island;
    rsx! {
        div {
            class: CLASS,
            IslandPagerCard {
                island,
            }
        }
    }
}

assert_component!(IslandPagerCardHost);
