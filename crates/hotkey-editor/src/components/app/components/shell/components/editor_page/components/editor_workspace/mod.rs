pub mod components;
mod props;
mod style;

use components::unit_detail::{UnitDetail, UnitDetailProps};
use components::unit_list::{UnitList, UnitListProps};
use dioxus::prelude::*;
pub use props::EditorWorkspaceProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(EditorWorkspace);

/// The editor's working area: the unit list laid out beside (or, on narrow widths,
/// stacked above) the unit detail panel. It owns the responsive two-column grid; the
/// active race is threaded to its descendants as a prop, so each colours itself.
#[component]
pub fn EditorWorkspace(props: EditorWorkspaceProps) -> Element {
    let class = CLASS;
    let unit_list = UnitListProps::from(&props);
    let unit_detail = UnitDetailProps::from(&props);
    rsx! {
        div {
            class,
            UnitList { ..unit_list }
            UnitDetail { ..unit_detail }
        }
    }
}
