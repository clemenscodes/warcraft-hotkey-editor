mod model;
mod view;

pub use view::PositionsContentView;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::island_detail::IslandDetail;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::island_sidebar::IslandSidebar;
use dioxus::prelude::*;
use model::PositionsContentModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PositionsContent(props: PositionsContentModel) -> Element {
    let sidebar_islands = props.islands.clone();
    let detail_islands = props.islands;
    rsx! {
        div {
            class: CLASS,
            IslandSidebar {
                islands: sidebar_islands,
            }
            IslandDetail {
                islands: detail_islands,
            }
        }
    }
}

assert_component!(PositionsContent);
