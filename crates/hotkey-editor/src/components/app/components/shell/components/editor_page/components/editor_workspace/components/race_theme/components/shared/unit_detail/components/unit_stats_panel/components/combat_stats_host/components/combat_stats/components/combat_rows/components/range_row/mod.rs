mod model;
mod view;

pub use view::RangeRowView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_label::StatLabel;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_value::StatValue;
use dioxus::prelude::*;
use model::RangeRowModel;
use style::CLASS;
use tw_macro::assert_component;

const LABEL_TEXT: &str = "Range";

#[component]
pub fn RangeRow(props: RangeRowModel) -> Element {
    let value = props.value;
    if value.is_zero() {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
            StatLabel {
                text: LABEL_TEXT,
            }
            StatValue {
                value,
            }
        }
    }
}

assert_component!(RangeRow);
