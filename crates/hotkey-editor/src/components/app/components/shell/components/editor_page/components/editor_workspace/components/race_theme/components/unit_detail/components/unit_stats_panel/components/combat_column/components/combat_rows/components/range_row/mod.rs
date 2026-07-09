mod props;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_stats_panel::components::shared::stat_label::StatLabel;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_stats_panel::components::shared::stat_value::StatValue;
use dioxus::prelude::*;
pub use props::RangeRowProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(RangeRow);

const LABEL_TEXT: &str = "Range";

/// The range row, shown only for a ranged attack. A melee attack has a reach of zero
/// and no row at all. A guarded leaf that early-returns.
#[component]
pub fn RangeRow(props: RangeRowProps) -> Element {
    let value = props.value;
    if value.is_zero() {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
            StatLabel { text: LABEL_TEXT }
            StatValue { value }
        }
    }
}
