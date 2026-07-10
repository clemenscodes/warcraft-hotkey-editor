pub mod components;
mod style;

use components::race_theme::RaceTheme;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The editor's working area: the unit list laid out beside (or, on narrow widths,
/// stacked above) the unit detail panel. It owns the responsive two-column grid and
/// nests the race-theme container, which publishes the active race's colour to the
/// list and detail panel it wraps. It threads nothing: the race theme and its children
/// read their state from context.
#[component]
pub fn EditorWorkspace() -> Element {
    let class = CLASS;
    rsx! {
        div {
            class,
            RaceTheme {}
        }
    }
}

assert_component!(EditorWorkspace);
