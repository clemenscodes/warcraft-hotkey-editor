pub mod components;
mod model;
mod view;

pub use view::SearchScopeMenuView;
mod style;

use components::search_scope_option::SearchScopeOption;
use dioxus::prelude::*;
use model::SearchScopeMenuModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SearchScopeMenu(props: SearchScopeMenuModel) -> Element {
    let unit_label = props.unit_label;
    let ability_label = props.ability_label;
    let unit_is_active = props.unit_is_active;
    let ability_is_active = props.ability_is_active;
    let select_unit = props.select_unit;
    let select_ability = props.select_ability;
    rsx! {
        div {
            class: CLASS,
            role: "listbox",
            SearchScopeOption {
                label: unit_label,
                is_active: unit_is_active,
                onclick: select_unit,
            }
            SearchScopeOption {
                label: ability_label,
                is_active: ability_is_active,
                onclick: select_ability,
            }
        }
    }
}

assert_component!(SearchScopeMenu);
