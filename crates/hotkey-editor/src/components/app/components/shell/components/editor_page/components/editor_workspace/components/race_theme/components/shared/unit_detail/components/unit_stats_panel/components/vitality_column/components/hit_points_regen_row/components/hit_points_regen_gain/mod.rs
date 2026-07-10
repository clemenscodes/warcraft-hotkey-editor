pub mod components;
mod logic;
mod props;

use components::active_hit_points_regen_gain::{
    ActiveHitPointsRegenGain, ActiveHitPointsRegenGainProps,
};
use components::muted_hit_points_regen_gain::{MutedHitPointsRegenGain, MutedHitPointsRegenGainProps};
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_figure::StatFigure;
use dioxus::prelude::*;
use tw_macro::assert_component;
pub use props::HitPointsRegenGainProps;

/// The health-regeneration gain: green, pushed to the row's end. A thin dispatcher —
/// it renders the active look (`ActiveHitPointsRegenGain`) xor the muted look
/// (`MutedHitPointsRegenGain`), each built by `From`, per whether the unit regenerates
/// health.
#[component]
pub fn HitPointsRegenGain(props: HitPointsRegenGainProps) -> Element {
    let is_muted = props.value.is_muted();
    if is_muted {
        let muted = MutedHitPointsRegenGainProps::from(&props);
        rsx! {
            MutedHitPointsRegenGain { ..muted }
        }
    } else {
        let active = ActiveHitPointsRegenGainProps::from(&props);
        rsx! {
            ActiveHitPointsRegenGain { ..active }
        }
    }
}

assert_component!(HitPointsRegenGain);
