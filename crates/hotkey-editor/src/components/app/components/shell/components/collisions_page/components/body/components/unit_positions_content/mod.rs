mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::unit_position_detail::UnitPositionDetail;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::unit_cards_sidebar::UnitCardsSidebar;
use dioxus::prelude::*;
use props::UnitPositionsContentPresentation;
pub use props::UnitPositionsContentProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnitPositionsContent);

/// The per-unit position-collision two-pane content: the clashing-units sidebar column
/// beside the fluid unit position detail pane.
#[component]
pub fn UnitPositionsContent(props: UnitPositionsContentProps) -> Element {
    let UnitPositionsContentPresentation {
        collision_kind,
        count,
    } = UnitPositionsContentPresentation::from(&props);
    let sidebar = props.sidebar;
    let detail = props.detail;
    rsx! {
        div {
            class: CLASS,
            "data-collision-kind": collision_kind,
            "data-unit-count": "{count}",
            UnitCardsSidebar { ..sidebar }
            UnitPositionDetail { ..detail }
        }
    }
}
