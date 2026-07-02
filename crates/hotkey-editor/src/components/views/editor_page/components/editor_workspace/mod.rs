mod props;
mod style;

use crate::assert_component;
use crate::components::unit_detail::{UnitDetailPanel, UnitDetailPanelProps};
use crate::components::unit_list::{UnitList, UnitListProps};
use dioxus::prelude::*;
pub use props::EditorWorkspaceProps;
assert_component!(EditorWorkspace);

/// The editor's working area: the unit list laid out beside (or, on narrow widths,
/// stacked above) the unit detail panel. It owns the responsive two-column grid and
/// carries the active-race accent that its descendants pick up.
#[component]
pub fn EditorWorkspace(props: EditorWorkspaceProps) -> Element {
    let race = *props.active_race.read();
    let class = style::class(race);
    let unit_list = UnitListProps::from(&props);
    let unit_detail = UnitDetailPanelProps::from(&props);
    rsx! {
        div {
            class,
            UnitList { ..unit_list }
            UnitDetailPanel { ..unit_detail }
        }
    }
}
