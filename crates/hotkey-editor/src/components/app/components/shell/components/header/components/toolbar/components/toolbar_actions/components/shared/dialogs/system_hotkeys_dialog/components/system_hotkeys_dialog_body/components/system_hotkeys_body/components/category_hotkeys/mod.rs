pub mod components;
mod model;
mod view;

pub use view::CategoryHotkeysView;

use components::control_groups_hotkeys_view::ControlGroupsHotkeysView;
use components::hero_selection_hotkeys_view::HeroSelectionHotkeysView;
use components::inventory_hotkeys_view::InventoryHotkeysView;
use components::system_hotkeys_list_view::SystemHotkeysListView;
use dioxus::prelude::*;
use model::CategoryHotkeysModel;
use tw_macro::assert_component;
use warcraft_api::SystemHotkeysCategory;

#[component]
pub fn CategoryHotkeys(props: CategoryHotkeysModel) -> Element {
    match props.category {
        SystemHotkeysCategory::Inventory => rsx! {
            InventoryHotkeysView {}
        },
        SystemHotkeysCategory::HeroSelection => rsx! {
            HeroSelectionHotkeysView {}
        },
        SystemHotkeysCategory::ControlGroups => rsx! {
            ControlGroupsHotkeysView {}
        },
        other_category => rsx! {
            SystemHotkeysListView {
                category: other_category,
            }
        },
    }
}

assert_component!(CategoryHotkeys);
