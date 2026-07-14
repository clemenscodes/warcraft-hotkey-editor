mod model;
mod presentation;
mod view;

pub use view::DefenseMatchupView;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::matchup::Matchup;
use dioxus::prelude::*;
use model::DefenseMatchupModel;
use presentation::{DefenseMatchupPresentation, use_defense_matchup};
use tw_macro::assert_component;

/// One cell of a defender's matchup. A thin wrapper over `Matchup`.
#[component]
pub fn DefenseMatchup(props: DefenseMatchupModel) -> Element {
    let DefenseMatchupPresentation {
        subject,
        multiplier,
        title,
        strength,
    } = use_defense_matchup(&props);
    rsx! {
        Matchup {
            subject,
            multiplier,
            title,
            strength,
        }
    }
}

assert_component!(DefenseMatchup);
