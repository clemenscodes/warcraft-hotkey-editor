mod model;
mod presentation;
mod view;

pub use view::AgilityRowView;

use crate::components::app::components::shell::components::editor_page::components::shared::unit_detail_trigger::components::unit_detail_dialog::components::unit_detail_dialog_body::components::unit_stats_panel::components::attributes_stats_host::components::attributes_stats::components::attribute_rows::components::shared::primary_attribute_row::PrimaryAttributeRow;
use crate::components::app::components::shell::components::editor_page::components::shared::unit_detail_trigger::components::unit_detail_dialog::components::unit_detail_dialog_body::components::unit_stats_panel::components::attributes_stats_host::components::attributes_stats::components::attribute_rows::components::shared::regular_attribute_row::RegularAttributeRow;
use dioxus::prelude::*;
use model::AgilityRowModel;
use presentation::{AgilityRowPresentation, use_agility_row};
use tw_macro::assert_component;

#[component]
pub fn AgilityRow(props: AgilityRowModel) -> Element {
    let AgilityRowPresentation {
        statistic,
        growth,
        label,
    } = use_agility_row(&props);
    if props.is_primary {
        rsx! {
            PrimaryAttributeRow {
                statistic,
                growth,
                label,
            }
        }
    } else {
        rsx! {
            RegularAttributeRow {
                statistic,
                growth,
                label,
            }
        }
    }
}

assert_component!(AgilityRow);
