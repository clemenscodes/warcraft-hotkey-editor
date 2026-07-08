pub mod components;
mod props;
mod style;

use components::race_theme::{RaceTheme, RaceThemeProps};
use dioxus::prelude::*;
pub use props::EditorWorkspaceProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(EditorWorkspace);

/// The editor's working area: the unit list laid out beside (or, on narrow widths,
/// stacked above) the unit detail panel. It owns the responsive two-column grid and
/// nests the race-theme container, which publishes the active race's colour to the
/// list and detail panel it wraps.
#[component]
pub fn EditorWorkspace(props: EditorWorkspaceProps) -> Element {
    let class = CLASS;
    let race_theme = RaceThemeProps::from(&props);
    rsx! {
        div {
            class,
            RaceTheme { ..race_theme }
        }
    }
}
