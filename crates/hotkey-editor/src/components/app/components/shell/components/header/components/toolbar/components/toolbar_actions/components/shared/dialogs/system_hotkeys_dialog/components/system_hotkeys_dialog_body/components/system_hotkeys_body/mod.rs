pub mod components;
mod presentation;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::control_groups_hotkeys_view::ControlGroupsHotkeysView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::hero_selection_hotkeys_view::HeroSelectionHotkeysView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::InventoryHotkeysView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::system_hotkeys_list_view::SystemHotkeysListView;
use dioxus::prelude::*;
use presentation::{use_system_hotkeys_body, SystemHotkeysBodyModel};
use warcraft_api::SystemHotkeysCategory;
use tw_macro::assert_component;

#[component]
pub fn SystemHotkeysBody() -> Element {
    let SystemHotkeysBodyModel { active_category } = use_system_hotkeys_body();
    let active = *active_category.read();
    match active {
        SystemHotkeysCategory::Inventory => {
            rsx! {
                InventoryHotkeysView {



                }
            }
        }
        SystemHotkeysCategory::HeroSelection => {
            rsx! {
                HeroSelectionHotkeysView {



                }
            }
        }
        SystemHotkeysCategory::ControlGroups => {
            rsx! {
                ControlGroupsHotkeysView {



                }
            }
        }
        other_category => {
            rsx! {
                SystemHotkeysListView {
                    category: other_category,
                }
            }
        }
    }
}

assert_component!(SystemHotkeysBody);
