mod props;

use crate::components::unit_detail::components::unit_stats_panel::components::matchup_cell::{
    MatchupCell, MatchupCellProps,
};
use dioxus::prelude::*;
pub use props::DefenseMatchupCellProps;

/// One cell of a defender's matchup. A thin wrapper over `MatchupCell`.
#[component]
pub fn DefenseMatchupCell(props: DefenseMatchupCellProps) -> Element {
    rsx! {
        MatchupCell { ..MatchupCellProps::from(&props) }
    }
}
