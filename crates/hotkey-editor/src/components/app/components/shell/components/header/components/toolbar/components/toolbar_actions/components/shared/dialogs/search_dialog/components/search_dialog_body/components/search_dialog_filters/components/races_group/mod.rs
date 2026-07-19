pub mod components;
mod data;
mod style;

use super::shared::filter_group_label::FilterGroupLabel;
use components::race_scope_menu::RaceScopeMenu;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn RacesGroup() -> Element {
    rsx! {
        div {
            class: CLASS,
            FilterGroupLabel {
                label: data::LABEL,
            }
            RaceScopeMenu {}
        }
    }
}

assert_component!(RacesGroup);
