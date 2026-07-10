pub mod components;
mod hooks;
mod style;

use components::mode_tabs::ModeTabs;
use dioxus::prelude::*;
use hooks::{ModeTabsInputs, use_mode_tabs_host};
use style::CLASS;
use tw_macro::assert_component;

/// Connected host for the mode column: sources the active mode and wires the select
/// handler (which dispatches the domain `select_mode` cascade), then hands the
/// presentational `ModeTabs` its two inputs. The tabs stay gallery-renderable.
#[component]
pub fn ModeTabsHost() -> Element {
    let ModeTabsInputs {
        unit_mode,
        on_select,
    } = use_mode_tabs_host();
    rsx! {
        div {
            class: CLASS,
            ModeTabs { unit_mode, on_select }
        }
    }
}

assert_component!(ModeTabsHost);
