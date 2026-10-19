mod model;
mod view;

pub use view::RangeRowView;

use crate::components::app::components::shell::components::editor_page::components::shared::unit_detail_trigger::components::unit_detail_dialog::components::unit_detail_dialog_body::components::unit_stats_panel::components::shared::stat_row::StatRow;
use dioxus::prelude::*;
use model::RangeRowModel;
use tw_macro::assert_component;

const LABEL_TEXT: &str = "Range";

#[component]
pub fn RangeRow(props: RangeRowModel) -> Element {
    let value = props.value;
    if value.is_zero() {
        return rsx! {};
    }
    rsx! {
        StatRow {
            label: LABEL_TEXT,
            value,
        }
    }
}

assert_component!(RangeRow);
