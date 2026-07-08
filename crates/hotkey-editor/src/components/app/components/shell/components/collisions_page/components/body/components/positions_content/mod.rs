mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::island_detail::IslandDetail;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::island_sidebar::IslandSidebar;
use dioxus::prelude::*;
use props::PositionsContentPresentation;
pub use props::PositionsContentProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(PositionsContent);

/// The position-collision two-pane content: the island sidebar column beside the
/// fluid island detail pane.
#[component]
pub fn PositionsContent(props: PositionsContentProps) -> Element {
    let PositionsContentPresentation {
        collision_kind,
        count,
    } = PositionsContentPresentation::from(&props);
    let sidebar = props.sidebar;
    let detail = props.detail;
    rsx! {
        div {
            class: CLASS,
            "data-collision-kind": collision_kind,
            "data-unit-count": "{count}",
            IslandSidebar { ..sidebar }
            IslandDetail { ..detail }
        }
    }
}
