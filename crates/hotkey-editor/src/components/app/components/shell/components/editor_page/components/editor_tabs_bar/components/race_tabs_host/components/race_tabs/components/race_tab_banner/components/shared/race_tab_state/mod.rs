pub mod components;
mod presentation;

use crate::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::race_tabs_host::components::race_tabs::RaceTabBinding;
use components::active_race_tab::ActiveRaceTab;
use components::inactive_race_tab::InactiveRaceTab;
use dioxus::prelude::*;
use presentation::RaceTabBehavior;
use tw_macro::assert_component;

/// The active-vs-inactive dispatcher for one race tab, shared by all five per-race
/// wrappers. It takes the finished `RaceTabBinding` (no navigation — the handlers are
/// already baked), shapes it into the tab's behaviour, then makes the clean switch:
/// `if active { ActiveRaceTab } else { InactiveRaceTab }`. Both variants render the same
/// base `RaceTab` by composition; the active one adds its accent on top. This dispatcher
/// owns no class, and there is no `data-active`: the look follows the component.
#[component]
pub fn RaceTabState(props: RaceTabBinding) -> Element {
    let behavior = RaceTabBehavior::from(&props);
    let label = behavior.label().to_string();
    let onclick = behavior.onclick();
    let onkeydown = behavior.onkeydown();
    match behavior.is_active() {
        true => rsx! {
            ActiveRaceTab {
                label,
                onclick,
                onkeydown,
            }
        },
        false => rsx! {
            InactiveRaceTab {
                label,
                onclick,
                onkeydown,
            }
        },
    }
}

assert_component!(RaceTabState);
