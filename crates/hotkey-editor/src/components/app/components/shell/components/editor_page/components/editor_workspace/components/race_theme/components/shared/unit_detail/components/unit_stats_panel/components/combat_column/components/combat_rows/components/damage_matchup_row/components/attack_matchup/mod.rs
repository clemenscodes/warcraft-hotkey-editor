mod props;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::matchup::{
    Matchup, MatchupProps,
};
use dioxus::prelude::*;
pub use props::AttackMatchupProps;

/// One cell of an attacker's damage matchup. A thin wrapper over `Matchup`.
use tw_macro::assert_component;
assert_component!(AttackMatchup);
#[component]
pub fn AttackMatchup(props: AttackMatchupProps) -> Element {
    rsx! {
        Matchup { ..MatchupProps::from(&props) }
    }
}
