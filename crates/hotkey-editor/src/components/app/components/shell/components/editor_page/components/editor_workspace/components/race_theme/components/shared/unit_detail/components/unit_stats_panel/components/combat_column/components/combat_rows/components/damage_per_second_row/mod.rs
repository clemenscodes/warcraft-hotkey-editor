mod props;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_label::StatLabel;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_value::StatValue;
use dioxus::prelude::*;
pub use props::DamagePerSecondRowProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(DamagePerSecondRow);

const LABEL_TEXT: &str = "Damage per Second";

/// The damage-per-second row, shown only when the attack has a real cooldown. A
/// guarded leaf that early-returns when the rate is undefined.
#[component]
pub fn DamagePerSecondRow(props: DamagePerSecondRowProps) -> Element {
    let Some(value) = props.value else {
        return rsx! {};
    };
    rsx! {
        div {
            class: CLASS,
            StatLabel { text: LABEL_TEXT }
            StatValue { value }
        }
    }
}
