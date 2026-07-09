mod props;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_stats_panel::components::shared::matchup::{
    Matchup, MatchupProps,
};
use dioxus::prelude::*;
pub use props::DefenseMatchupProps;

/// One cell of a defender's matchup. A thin wrapper over `Matchup`.
use tw_macro::assert_component;
assert_component!(DefenseMatchup);
#[component]
pub fn DefenseMatchup(props: DefenseMatchupProps) -> Element {
    rsx! {
        Matchup { ..MatchupProps::from(&props) }
    }
}
