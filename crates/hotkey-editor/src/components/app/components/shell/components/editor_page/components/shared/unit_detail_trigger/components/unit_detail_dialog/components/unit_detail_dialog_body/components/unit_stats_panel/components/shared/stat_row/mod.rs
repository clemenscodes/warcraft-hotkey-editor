mod model;
mod style;
mod view;

pub use view::StatRowView;

use crate::components::app::components::shell::components::editor_page::components::shared::unit_detail_trigger::components::unit_detail_dialog::components::unit_detail_dialog_body::components::unit_stats_panel::components::shared::stat_figure::StatFigure;
use crate::components::app::components::shell::components::editor_page::components::shared::unit_detail_trigger::components::unit_detail_dialog::components::unit_detail_dialog_body::components::unit_stats_panel::components::shared::stat_label::StatLabel;
use crate::components::app::components::shell::components::editor_page::components::shared::unit_detail_trigger::components::unit_detail_dialog::components::unit_detail_dialog_body::components::unit_stats_panel::components::shared::stat_value::StatValue;
use dioxus::prelude::*;
use model::StatRowModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn StatRow<Figure: StatFigure>(props: StatRowModel<Figure>) -> Element {
    let label = props.label;
    let value = props.value;
    rsx! {
        div {
            class: CLASS,
            StatLabel {
                text: label,
            }
            StatValue {
                value,
            }
        }
    }
}

assert_component!(StatRow);
