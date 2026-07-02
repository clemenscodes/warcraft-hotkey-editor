mod props;

use crate::components::unit_detail::components::unit_stats_panel::components::matchup_cell::{
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
