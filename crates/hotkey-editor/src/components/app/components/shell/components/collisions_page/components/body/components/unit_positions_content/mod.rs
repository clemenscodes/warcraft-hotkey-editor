mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::unit_position_detail::UnitPositionDetail;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::unit_cards_sidebar::UnitCardsSidebar;
use dioxus::prelude::*;
use props::UnitPositionsContentProps;
use style::CLASS;
use tw_macro::assert_component;

/// The per-unit position-collision two-pane content: the clashing-units sidebar column
/// beside the fluid unit position detail pane.
#[component]
pub fn UnitPositionsContent(props: UnitPositionsContentProps) -> Element {
    let sidebar_units = props.units.clone();
    let detail_units = props.units;
    rsx! {
        div {
            class: CLASS,
            UnitCardsSidebar { units: sidebar_units }
            UnitPositionDetail { units: detail_units }
        }
    }
}

assert_component!(UnitPositionsContent);
