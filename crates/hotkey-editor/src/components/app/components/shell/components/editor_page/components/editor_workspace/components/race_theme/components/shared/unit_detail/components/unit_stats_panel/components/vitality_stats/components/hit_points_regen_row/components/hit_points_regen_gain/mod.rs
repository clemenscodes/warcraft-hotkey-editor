pub mod components;
mod model;
mod view;

use components::active_hit_points_regen_gain::ActiveHitPointsRegenGain;
use components::muted_hit_points_regen_gain::MutedHitPointsRegenGain;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_figure::StatFigure;
use dioxus::prelude::*;
use tw_macro::assert_component;
use model::HitPointsRegenGainModel;

/// The health-regeneration gain: green, pushed to the row's end. A thin dispatcher —
/// it renders the active look (`ActiveHitPointsRegenGain`) xor the muted look
/// (`MutedHitPointsRegenGain`), per whether the unit regenerates health.
#[component]
pub fn HitPointsRegenGain(props: HitPointsRegenGainModel) -> Element {
    let value = props.value;
    let is_muted = value.is_muted();
    let text = value.display();
    if is_muted {
        rsx! {
            MutedHitPointsRegenGain {
                text,
            }
        }
    } else {
        rsx! {
            ActiveHitPointsRegenGain {
                text,
            }
        }
    }
}

assert_component!(HitPointsRegenGain);
