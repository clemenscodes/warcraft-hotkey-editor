pub mod components;
mod presentation;
mod style;

use components::race_scope_backdrop::RaceScopeBackdrop;
use components::race_scope_panel::RaceScopePanel;
use components::race_scope_trigger::RaceScopeTrigger;
use dioxus::prelude::*;
use presentation::{RaceScopeMenuPresentation, use_race_scope_menu};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn RaceScopeMenu() -> Element {
    let RaceScopeMenuPresentation {
        summary,
        is_open,
        toggle,
        dismiss,
    } = use_race_scope_menu();
    rsx! {
        div {
            class: CLASS,
            RaceScopeTrigger {
                summary,
                is_open,
                onclick: toggle,
            }
            if is_open {
                RaceScopePanel {
                    on_back: dismiss,
                }
                RaceScopeBackdrop {
                    onclick: dismiss,
                }
            }
        }
    }
}

assert_component!(RaceScopeMenu);
