pub mod components;
mod presentation;
mod style;

use components::mode_tabs::ModeTabs;
use dioxus::prelude::*;
use presentation::{ModeTabsInputs, use_mode_tabs_host};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ModeTabsHost() -> Element {
    let ModeTabsInputs {
        unit_modes,
        on_select,
    } = use_mode_tabs_host();
    rsx! {
        div {
            class: CLASS,
            ModeTabs {
                unit_modes,
                on_select,
            }
        }
    }
}

assert_component!(ModeTabsHost);
