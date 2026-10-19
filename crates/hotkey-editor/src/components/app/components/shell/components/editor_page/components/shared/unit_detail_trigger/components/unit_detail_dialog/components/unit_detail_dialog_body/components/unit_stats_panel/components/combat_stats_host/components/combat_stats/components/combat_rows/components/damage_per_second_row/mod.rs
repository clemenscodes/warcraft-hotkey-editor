mod model;
mod view;

pub use view::DamagePerSecondRowView;

use crate::components::app::components::shell::components::editor_page::components::shared::unit_detail_trigger::components::unit_detail_dialog::components::unit_detail_dialog_body::components::unit_stats_panel::components::shared::stat_row::StatRow;
use dioxus::prelude::*;
use model::DamagePerSecondRowModel;
use tw_macro::assert_component;

const LABEL_TEXT: &str = "Damage per Second";

#[component]
pub fn DamagePerSecondRow(props: DamagePerSecondRowModel) -> Element {
    let Some(value) = props.value else {
        return rsx! {};
    };
    rsx! {
        StatRow {
            label: LABEL_TEXT,
            value,
        }
    }
}

assert_component!(DamagePerSecondRow);
