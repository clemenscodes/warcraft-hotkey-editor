pub mod components;
mod data;
mod model;
mod presentation;
mod style;
mod view;

pub use view::SearchScopePickerView;

use components::search_scope_backdrop::SearchScopeBackdrop;
use components::search_scope_menu::SearchScopeMenu;
use components::search_scope_trigger::SearchScopeTrigger;
use dioxus::prelude::*;
use presentation::{SearchScopePickerPresentation, use_search_scope_picker};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SearchScopePicker() -> Element {
    let SearchScopePickerPresentation {
        is_open,
        current_label,
        toggle,
        dismiss,
        unit_is_active,
        ability_is_active,
        select_unit,
        select_ability,
    } = use_search_scope_picker();
    rsx! {
        div {
            class: CLASS,
            SearchScopeTrigger {
                label: current_label,
                is_open,
                onclick: toggle,
            }
            if is_open {
                SearchScopeMenu {
                    unit_label: data::UNIT,
                    ability_label: data::ABILITY,
                    unit_is_active,
                    ability_is_active,
                    select_unit,
                    select_ability,
                }
                SearchScopeBackdrop {
                    onclick: dismiss,
                }
            }
        }
    }
}

assert_component!(SearchScopePicker);
