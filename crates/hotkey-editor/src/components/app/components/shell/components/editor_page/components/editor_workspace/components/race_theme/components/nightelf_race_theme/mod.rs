mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::UnitDetail;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_list::UnitList;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The Night Elf race-themed container. A `display:contents` wrapper that publishes the
/// Night Elf colour as `--race-color` for its descendants — the unit
/// list and the unit detail panel — which read them for their selected and hover accents.
/// Being `contents`, it adds no box: the custom properties inherit down while the
/// workspace grid still lays out the list and detail directly.
#[component]
pub fn NightelfRaceTheme() -> Element {
    rsx! {
        div {
            class: CLASS,
            UnitList {}
            UnitDetail {}
        }
    }
}

assert_component!(NightelfRaceTheme);
