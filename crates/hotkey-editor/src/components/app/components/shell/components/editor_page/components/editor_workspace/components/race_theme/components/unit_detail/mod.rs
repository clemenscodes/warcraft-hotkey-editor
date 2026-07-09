pub mod components;
mod hooks;
mod logic;
mod props;
mod state;
mod style;

use components::unit_description::UnitDescription;
use components::unit_detail_body::UnitDetailBody;
use components::unit_detail_empty::UnitDetailEmpty;
use components::unit_detail_header::UnitDetailHeader;
use components::unit_stats_panel::UnitStatsPanel;
use dioxus::prelude::*;
use hooks::use_unit_detail_panel;
use state::UnitDetailView;
use style::CLASS;

/// The unit detail card: header, description, stats, and the grids/override body.
/// Empty until a unit is selected. Composes its children from a single shaped hook
/// that reads the active race and selected unit from context, so the panel takes no
/// props.
use tw_macro::assert_component;
assert_component!(UnitDetail);
#[component]
pub fn UnitDetail() -> Element {
    let model = match use_unit_detail_panel() {
        UnitDetailView::Loaded(model) => *model,
        UnitDetailView::Empty(message) => {
            return rsx! {
                UnitDetailEmpty { message }
            };
        }
    };
    rsx! {
        section {
            class: CLASS,
            UnitDetailHeader { ..model.header }
            UnitDescription { ..model.description }
            UnitStatsPanel { ..model.stats }
            UnitDetailBody { ..model.body }
        }
    }
}
