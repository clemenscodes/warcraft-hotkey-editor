pub mod components;
mod hooks;
mod logic;

use crate::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::race_tabs_host::components::race_tabs::RaceTabBinding;
use components::active_race_tab::{ActiveRaceTab, ActiveRaceTabProps};
use components::inactive_race_tab::{InactiveRaceTab, InactiveRaceTabProps};
use dioxus::prelude::*;
use hooks::use_race_tab;
use tw_macro::assert_component;
assert_component!(RaceTabState);

/// The active-vs-inactive dispatcher for one race tab, shared by all five per-race
/// wrappers. It takes the finished `RaceTabBinding` (no navigation — the handlers are
/// already baked), adds the per-tab mount/focus behaviour, then makes the clean switch:
/// `if active { ActiveRaceTab } else { InactiveRaceTab }`. Both variants render the same
/// base `RaceTab` by composition; the active one adds its accent on top. This dispatcher
/// owns no class, and there is no `data-active`: the look follows the component.
#[component]
pub fn RaceTabState(props: RaceTabBinding) -> Element {
    let behavior = use_race_tab(props);
    match behavior.is_active() {
        true => {
            let active = ActiveRaceTabProps::from(&behavior);
            rsx! {
                ActiveRaceTab { ..active }
            }
        }
        false => {
            let inactive = InactiveRaceTabProps::from(&behavior);
            rsx! {
                InactiveRaceTab { ..inactive }
            }
        }
    }
}
