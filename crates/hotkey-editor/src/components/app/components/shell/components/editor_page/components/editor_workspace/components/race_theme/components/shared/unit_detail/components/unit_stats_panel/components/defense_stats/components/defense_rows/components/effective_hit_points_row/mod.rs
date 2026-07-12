mod model;
mod view;

pub use view::EffectiveHitPointsRowView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_label::StatLabel;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_value::StatValue;
use dioxus::prelude::*;
use model::EffectiveHitPointsRowModel;
use style::CLASS;
use tw_macro::assert_component;

const LABEL_TEXT: &str = "Effective Hit Points";

/// The unit's effective hit points — raw health scaled by armor mitigation.
#[component]
pub fn EffectiveHitPointsRow(props: EffectiveHitPointsRowModel) -> Element {
    let value = props.value;
    rsx! {
        div {
            class: CLASS,
            StatLabel { text: LABEL_TEXT }
            StatValue { value }
        }
    }
}

assert_component!(EffectiveHitPointsRow);
