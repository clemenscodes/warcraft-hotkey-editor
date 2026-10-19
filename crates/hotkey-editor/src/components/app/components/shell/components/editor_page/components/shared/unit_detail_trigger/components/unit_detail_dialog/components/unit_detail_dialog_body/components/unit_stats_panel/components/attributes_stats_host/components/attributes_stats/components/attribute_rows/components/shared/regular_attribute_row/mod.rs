mod model;
mod view;

pub use view::RegularAttributeRowView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::shared::unit_detail_trigger::components::unit_detail_dialog::components::unit_detail_dialog_body::components::unit_stats_panel::components::attributes_stats_host::components::attributes_stats::components::attribute_rows::components::shared::primary_attribute_label::PrimaryAttributeLabel;
use crate::components::app::components::shell::components::editor_page::components::shared::unit_detail_trigger::components::unit_detail_dialog::components::unit_detail_dialog_body::components::unit_stats_panel::components::shared::stat_gain::StatGain;
use crate::components::app::components::shell::components::editor_page::components::shared::unit_detail_trigger::components::unit_detail_dialog::components::unit_detail_dialog_body::components::unit_stats_panel::components::shared::stat_value::StatValue;
use dioxus::prelude::*;
use model::RegularAttributeRowModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn RegularAttributeRow(props: RegularAttributeRowModel) -> Element {
    let statistic = props.statistic;
    let growth = props.growth;
    let label = props.label;
    rsx! {
        div {
            class: CLASS,
            PrimaryAttributeLabel {
                text: label,
            }
            StatValue {
                value: statistic,
            }
            StatGain {
                value: growth,
            }
        }
    }
}

assert_component!(RegularAttributeRow);
