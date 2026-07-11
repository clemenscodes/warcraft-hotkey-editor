mod model;
mod presentation;
mod view;

pub use view::AttackMatchupView;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::matchup::Matchup;
use dioxus::prelude::*;
use model::AttackMatchupModel;
use presentation::{AttackMatchupPresentation, use_attack_matchup};
use tw_macro::assert_component;

/// One cell of an attacker's damage matchup. A thin wrapper over `Matchup`.
#[component]
pub fn AttackMatchup(props: AttackMatchupModel) -> Element {
    let AttackMatchupPresentation {
        subject,
        multiplier,
        title,
        strength,
    } = use_attack_matchup(&props);
    rsx! {
        Matchup { subject, multiplier, title, strength }
    }
}

assert_component!(AttackMatchup);
