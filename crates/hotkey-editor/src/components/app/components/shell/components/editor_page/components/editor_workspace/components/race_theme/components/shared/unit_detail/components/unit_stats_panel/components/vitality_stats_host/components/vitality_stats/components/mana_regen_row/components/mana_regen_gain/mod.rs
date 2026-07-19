pub mod components;
mod model;
mod view;

use components::active_mana_regen_gain::ActiveManaRegenGain;
use components::muted_mana_regen_gain::MutedManaRegenGain;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_figure::StatFigure;
use dioxus::prelude::*;
use tw_macro::assert_component;
use model::ManaRegenGainModel;

#[component]
pub fn ManaRegenGain(props: ManaRegenGainModel) -> Element {
    let value = props.value;
    let is_muted = value.is_muted();
    let text = value.display();
    if is_muted {
        rsx! {
            MutedManaRegenGain {
                text,
            }
        }
    } else {
        rsx! {
            ActiveManaRegenGain {
                text,
            }
        }
    }
}

assert_component!(ManaRegenGain);
