pub mod components;
mod props;

use components::active_mana_value::ActiveManaValue;
use components::muted_mana_value::MutedManaValue;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_figure::StatFigure;
use dioxus::prelude::*;
use tw_macro::assert_component;
use props::ManaValueProps;

/// The mana figure: the human-blue accent, semibold and enlarged when the unit has a
/// mana pool, faint when it has none. A thin dispatcher — it renders the active look
/// (`ActiveManaValue`) xor the muted look (`MutedManaValue`).
#[component]
pub fn ManaValue(props: ManaValueProps) -> Element {
    let value = props.value;
    let is_muted = value.is_muted();
    let text = value.display();
    if is_muted {
        rsx! {
            MutedManaValue { text }
        }
    } else {
        rsx! {
            ActiveManaValue { text }
        }
    }
}

assert_component!(ManaValue);
