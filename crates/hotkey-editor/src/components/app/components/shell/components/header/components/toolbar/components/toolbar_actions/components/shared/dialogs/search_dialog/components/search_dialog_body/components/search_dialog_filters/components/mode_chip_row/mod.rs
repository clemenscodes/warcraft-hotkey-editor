mod presentation;
mod style;

use crate::components::app::components::shell::components::editor_page::components::shared::toggle_button::ToggleButton;
use dioxus::prelude::*;
use presentation::use_mode_chip_row;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ModeChipRow() -> Element {
    let toggles = use_mode_chip_row();
    rsx! {
        div {
            class: CLASS,
            role: "group",
            aria_label: "Narrow the catalog",
            for toggle in toggles {
                ToggleButton {
                    key: "{toggle.key}",
                    label: toggle.label,
                    title: toggle.title,
                    active: toggle.is_active,
                    onclick: toggle.on_pick,
                }
            }
        }
    }
}

assert_component!(ModeChipRow);
