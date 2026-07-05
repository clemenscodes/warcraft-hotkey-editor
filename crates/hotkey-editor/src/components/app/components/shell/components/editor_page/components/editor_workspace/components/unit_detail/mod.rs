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
pub use props::UnitDetailPanelProps;
use state::UnitDetailView;
use style::CLASS;

/// The unit detail card: header, description, stats, and the grids/override body.
/// Empty until a unit is selected. Composes its children from a single shaped hook.
#[component]
pub fn UnitDetailPanel(props: UnitDetailPanelProps) -> Element {
    let model = match use_unit_detail_panel(&props) {
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
