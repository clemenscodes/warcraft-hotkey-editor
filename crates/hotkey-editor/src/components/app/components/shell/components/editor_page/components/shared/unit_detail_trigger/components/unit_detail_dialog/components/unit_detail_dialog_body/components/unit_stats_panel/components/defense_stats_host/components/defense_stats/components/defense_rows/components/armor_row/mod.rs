mod model;
mod view;

pub use view::ArmorRowView;

use crate::components::app::components::shell::components::editor_page::components::shared::unit_detail_trigger::components::unit_detail_dialog::components::unit_detail_dialog_body::components::unit_stats_panel::components::shared::stat_row::StatRow;
use dioxus::prelude::*;
use model::ArmorRowModel;
use tw_macro::assert_component;

const LABEL_TEXT: &str = "Armor";

#[component]
pub fn ArmorRow(props: ArmorRowModel) -> Element {
    let value = props.value;
    rsx! {
        StatRow {
            label: LABEL_TEXT,
            value,
        }
    }
}

assert_component!(ArmorRow);
