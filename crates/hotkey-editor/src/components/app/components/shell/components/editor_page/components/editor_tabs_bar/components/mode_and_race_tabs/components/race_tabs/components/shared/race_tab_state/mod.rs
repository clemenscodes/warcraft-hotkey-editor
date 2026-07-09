pub mod components;
mod hooks;
mod logic;
mod props;

use components::active_race_tab::{ActiveRaceTab, ActiveRaceTabProps};
use components::inactive_race_tab::{InactiveRaceTab, InactiveRaceTabProps};
use dioxus::prelude::*;
use hooks::use_race_tab;
pub use props::RaceTabStateProps;
use tw_macro::assert_component;
assert_component!(RaceTabState);

/// The active-vs-inactive dispatcher for one race tab, shared by all five per-race
/// wrappers. It calls the shared behaviour hook once — building the tab's activation
/// handlers and reading whether its race is the active one — then makes the clean switch:
/// `if active { ActiveRaceTab } else { InactiveRaceTab }`. Both variants render the same
/// base `RaceTab` by composition; the active one adds its accent on top. This dispatcher
/// owns no class. There is no `data-active`: the look follows the component.
#[component]
pub fn RaceTabState(props: RaceTabStateProps) -> Element {
    let tab = use_race_tab(props);
    match tab.is_active() {
        true => {
            let active = ActiveRaceTabProps::from(&tab);
            rsx! {
                ActiveRaceTab { ..active }
            }
        }
        false => {
            let inactive = InactiveRaceTabProps::from(&tab);
            rsx! {
                InactiveRaceTab { ..inactive }
            }
        }
    }
}
