mod hooks;
mod props;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::matchup::Matchup;
use dioxus::prelude::*;
use hooks::{DefenseMatchupModel, use_defense_matchup};
use props::DefenseMatchupProps;
use tw_macro::assert_component;

/// One cell of a defender's matchup. A thin wrapper over `Matchup`.
#[component]
pub fn DefenseMatchup(props: DefenseMatchupProps) -> Element {
    let DefenseMatchupModel {
        subject,
        multiplier,
        title,
        strength,
    } = use_defense_matchup(&props);
    rsx! {
        Matchup { subject, multiplier, title, strength }
    }
}

assert_component!(DefenseMatchup);
