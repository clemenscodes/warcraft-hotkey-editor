mod derived_stats;
mod hooks;
mod props;
mod style;
pub mod tile_override_empty;
pub mod tile_override_panel;
pub mod unit_command_grids;
pub mod unit_description;
pub mod unit_detail_body;
pub mod unit_detail_empty;
pub mod unit_detail_header;
pub mod unit_detail_row;
pub mod unit_stats_panel;
pub mod unit_tile_override;

use dioxus::prelude::*;
use hooks::{UnitDetailView, use_unit_detail_panel};
pub use props::UnitDetailPanelProps;
use style::CLASS;
use unit_description::UnitDescription;
use unit_detail_body::UnitDetailBody;
use unit_detail_empty::UnitDetailEmpty;
use unit_detail_header::UnitDetailHeader;
use unit_stats_panel::UnitStatsPanel;

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
