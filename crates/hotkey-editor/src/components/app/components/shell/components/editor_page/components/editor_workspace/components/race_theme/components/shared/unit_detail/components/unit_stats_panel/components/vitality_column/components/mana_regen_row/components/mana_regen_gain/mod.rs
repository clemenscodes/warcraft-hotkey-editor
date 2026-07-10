pub mod components;
mod logic;
mod props;

use components::active_mana_regen_gain::{ActiveManaRegenGain, ActiveManaRegenGainProps};
use components::muted_mana_regen_gain::{MutedManaRegenGain, MutedManaRegenGainProps};
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_figure::StatFigure;
use dioxus::prelude::*;
use tw_macro::assert_component;
pub use props::ManaRegenGainProps;

/// The mana-regeneration gain: the human-blue accent, pushed to the row's end. A thin
/// dispatcher — it renders the active look (`ActiveManaRegenGain`) xor the muted look
/// (`MutedManaRegenGain`), each built by `From`, per whether the unit regenerates mana.
#[component]
pub fn ManaRegenGain(props: ManaRegenGainProps) -> Element {
    let is_muted = props.value.is_muted();
    if is_muted {
        let muted = MutedManaRegenGainProps::from(&props);
        rsx! {
            MutedManaRegenGain { ..muted }
        }
    } else {
        let active = ActiveManaRegenGainProps::from(&props);
        rsx! {
            ActiveManaRegenGain { ..active }
        }
    }
}

assert_component!(ManaRegenGain);
