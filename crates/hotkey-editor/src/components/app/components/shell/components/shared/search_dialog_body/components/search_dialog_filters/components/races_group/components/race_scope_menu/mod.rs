pub mod components;
mod presentation;
mod style;

use crate::components::app::components::shell::components::shared::search_dialog_body::components::search_dialog_filters::components::shared::popover_backdrop::PopoverBackdrop;
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
        is_mounted,
        is_leaving,
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
            if is_mounted {
                RaceScopePanel {
                    leaving: is_leaving,
                }
            }
            if is_open {
                PopoverBackdrop {
                    onclick: dismiss,
                }
            }
        }
    }
}

assert_component!(RaceScopeMenu);
