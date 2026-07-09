pub mod components;
mod hooks;
mod style;

use components::race_tabs::RaceTabs;
use dioxus::prelude::*;
use hooks::use_race_tabs_host;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(RaceTabsHost);

/// Connected host for the race tabs: sources the active race and wires the select
/// handler (which dispatches the domain `select_race` cascade), then hands the
/// presentational `RaceTabs` its two inputs. The tabs stay gallery-renderable because
/// they take those inputs as props; only this host reaches for context.
#[component]
pub fn RaceTabsHost() -> Element {
    let race_tabs = use_race_tabs_host();
    rsx! {
        div {
            class: CLASS,
            RaceTabs { ..race_tabs }
        }
    }
}
