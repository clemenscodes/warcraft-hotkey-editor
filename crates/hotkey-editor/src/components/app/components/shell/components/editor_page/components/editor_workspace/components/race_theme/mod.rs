pub mod components;
mod style;

use crate::services::navigation::context::use_view_navigation;
use components::unit_detail::UnitDetail;
use components::unit_list::UnitList;
use dioxus::prelude::*;
use tw_macro::assert_component;
assert_component!(RaceTheme);

/// Publishes the active race's colour as `--race-color` for the race-themed content —
/// the unit list and the unit detail panel. A `display:contents` wrapper, so it adds no
/// box of its own: the custom property inherits down to the cards, which read it for
/// their selected/hover accent, while the workspace grid still lays out the list and
/// detail directly. It reads the active race from context, so it takes no props, and its
/// children source their own state from context too.
#[component]
pub fn RaceTheme() -> Element {
    let race = *use_view_navigation().active_race().read();
    let class = style::theme(race);
    rsx! {
        div {
            class,
            UnitList {}
            UnitDetail {}
        }
    }
}
