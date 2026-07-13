pub mod components;
mod model;
mod presentation;
mod view;

pub use view::RaceTabsView;
mod style;

use components::race_tab_banner::RaceTabBanner;
use dioxus::prelude::*;
use model::RaceTabBinding;
use model::RaceTabsModel;
use presentation::RaceTabsPresentation;
use style::CLASS;
use tw_macro::assert_component;

/// The race tabs: one banner tab per race, iterated from the domain's `AllRaces` set so
/// the renderer never hardcodes which races exist. It is the one consumer of the active
/// race — the presentation reads it once, marks each tab active, and bakes the select
/// handler into each tab's handlers — so only finished, navigation-free items flow to
/// the per-race dispatcher. Each per-race wrapper adds only its own colours on top.
#[component]
pub fn RaceTabs(props: RaceTabsModel) -> Element {
    let RaceTabsPresentation { tabs } = RaceTabsPresentation::from(&props);
    rsx! {
        nav {
            class: CLASS,
            for tab in tabs {
                RaceTabBanner {
                    race: tab.race,
                    is_active: tab.is_active,
                    label: tab.label,
                    onclick: tab.onclick,
                    onkeydown: tab.onkeydown,
                }
            }
        }
    }
}

assert_component!(RaceTabs);
