mod props;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_stats_panel::components::shared::matchup_cell::{
    MatchupCell, MatchupCellProps,
};
use dioxus::prelude::*;
pub use props::AttackMatchupCellProps;

/// One cell of an attacker's damage matchup. A thin wrapper over `MatchupCell`.
#[component]
pub fn AttackMatchupCell(props: AttackMatchupCellProps) -> Element {
    rsx! {
        MatchupCell { ..MatchupCellProps::from(&props) }
    }
}
