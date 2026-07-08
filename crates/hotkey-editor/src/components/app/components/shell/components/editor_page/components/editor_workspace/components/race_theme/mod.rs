pub mod components;
mod props;
mod style;

use components::unit_detail::{UnitDetail, UnitDetailProps};
use components::unit_list::{UnitList, UnitListProps};
use dioxus::prelude::*;
pub use props::RaceThemeProps;
use tw_macro::assert_component;
assert_component!(RaceTheme);

/// Publishes the active race's colour as `--race-color` for the race-themed content —
/// the unit list and the unit detail panel — then hands each child its own props. A
/// `display:contents` wrapper, so it adds no box of its own: the custom property
/// inherits down to the cards, which read it for their selected/hover accent, while
/// the workspace grid still lays out the list and detail directly.
#[component]
pub fn RaceTheme(props: RaceThemeProps) -> Element {
    let race = *props.active_race.read();
    let class = style::theme(race);
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
