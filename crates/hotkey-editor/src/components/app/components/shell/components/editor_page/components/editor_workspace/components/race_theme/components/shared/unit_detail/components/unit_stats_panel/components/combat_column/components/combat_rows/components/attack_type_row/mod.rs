mod props;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_label::StatLabel;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_value::StatValue;
use dioxus::prelude::*;
pub use props::AttackTypeRowProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(AttackTypeRow);

const LABEL_TEXT: &str = "Attack Type";

/// The unit's attack type — what its damage is classified as.
#[component]
pub fn AttackTypeRow(props: AttackTypeRowProps) -> Element {
    let value = props.value;
    rsx! {
        div {
            class: CLASS,
            StatLabel { text: LABEL_TEXT }
            StatValue { value }
        }
    }
}
