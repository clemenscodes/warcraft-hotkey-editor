mod model;
mod presentation;
mod view;

pub use view::RaceTabsView;
mod style;

use crate::components::app::components::shell::components::shared::race_tab_banner::RaceTabBanner;
use dioxus::prelude::*;
use model::RaceTabsModel;
use presentation::RaceTabsPresentation;
use style::CLASS;
use tw_macro::assert_component;

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
