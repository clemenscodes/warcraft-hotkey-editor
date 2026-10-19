pub mod components;
mod model;
mod presentation;
mod style;

use components::active_accent::ActiveAccent;
use components::race_tab::RaceTab;
use dioxus::prelude::*;
use model::RaceTabStateModel;
use presentation::RaceTabStatePresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn RaceTabState(props: RaceTabStateModel) -> Element {
    let RaceTabStatePresentation {
        label,
        onclick,
        onkeydown,
        accent_hidden,
    } = RaceTabStatePresentation::from(&props);
    // The tab and its accent stay mounted whichever race is active; only the
    // accent's visibility changes, so it fades in and out, and the label takes
    // the race colour from the visible accent.
    rsx! {
        div {
            class: CLASS,
            RaceTab {
                label,
                onclick,
                onkeydown,
            }
            ActiveAccent {
                hidden: accent_hidden,
            }
        }
    }
}

assert_component!(RaceTabState);
