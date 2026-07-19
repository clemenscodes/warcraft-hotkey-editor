pub mod components;
mod data;
mod presentation;
mod style;

use super::shared::filter_group_label::FilterGroupLabel;
use components::also_include_switch::AlsoIncludeSwitch;
use dioxus::prelude::*;
use presentation::use_also_include_group;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn AlsoIncludeGroup() -> Element {
    let switches = use_also_include_group();
    rsx! {
        div {
            class: CLASS,
            FilterGroupLabel {
                label: data::LABEL,
            }
            for switch in switches {
                AlsoIncludeSwitch {
                    key: "{switch.key}",
                    label: switch.label,
                    popover_text: switch.popover_text,
                    is_on: switch.is_on,
                    onclick: switch.onclick,
                }
            }
        }
    }
}

assert_component!(AlsoIncludeGroup);
