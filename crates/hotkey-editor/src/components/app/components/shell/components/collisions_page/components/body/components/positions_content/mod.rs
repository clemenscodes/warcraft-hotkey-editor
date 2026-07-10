mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::island_detail::IslandDetail;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::island_sidebar::IslandSidebar;
use dioxus::prelude::*;
use props::PositionsContentProps;
use style::CLASS;
use tw_macro::assert_component;

/// The position-collision two-pane content: the island sidebar column beside the
/// fluid island detail pane.
#[component]
pub fn PositionsContent(props: PositionsContentProps) -> Element {
    let sidebar_islands = props.islands.clone();
    let detail_islands = props.islands;
    rsx! {
        div {
            class: CLASS,
            IslandSidebar { islands: sidebar_islands }
            IslandDetail { islands: detail_islands }
        }
    }
}

assert_component!(PositionsContent);
