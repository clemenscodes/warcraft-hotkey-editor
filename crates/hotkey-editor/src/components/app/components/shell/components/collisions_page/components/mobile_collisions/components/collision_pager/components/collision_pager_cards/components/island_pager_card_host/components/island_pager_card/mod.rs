mod model;
mod view;

pub use view::IslandPagerCardView;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::island_detail::components::island_detail_body::components::filled_island_detail::FilledIslandDetail;
use dioxus::prelude::*;
use model::IslandPagerCardModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn IslandPagerCard(props: IslandPagerCardModel) -> Element {
    let island = props.island;
    rsx! {
        div {
            class: CLASS,
            FilledIslandDetail {
                island,
            }
        }
    }
}

assert_component!(IslandPagerCard);
