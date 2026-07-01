mod props;

use crate::assert_component;
use crate::components::dialogs::system_hotkeys_dialog::components::control_groups_hotkeys_view::ControlGroupsHotkeysView;
use crate::components::dialogs::system_hotkeys_dialog::components::hero_selection_hotkeys_view::HeroSelectionHotkeysView;
use crate::components::dialogs::system_hotkeys_dialog::components::inventory_hotkeys_view::InventoryHotkeysView;
use crate::components::dialogs::system_hotkeys_dialog::components::system_hotkeys_list_view::SystemHotkeysListView;
use dioxus::prelude::*;
pub use props::SystemHotkeysBodyProps;
use warcraft_database::SystemHotkeysCategory;
assert_component!(SystemHotkeysBody);

/// Renders the editor for the active category. Inventory, hero selection, and
/// control groups have bespoke editors; every other category is a plain list of
/// key rows. A pure selector: it holds no class and picks one child to render.
#[component]
pub fn SystemHotkeysBody(props: SystemHotkeysBodyProps) -> Element {
    let loaded_keys = props.loaded_keys;
    let editing_section = props.editing_section;
    let drag_follower = props.drag_follower;
    let active = *props.active_category.read();
    match active {
        SystemHotkeysCategory::Inventory => {
            rsx! {
                InventoryHotkeysView { loaded_keys, editing_section, drag_follower }
            }
        }
        SystemHotkeysCategory::HeroSelection => {
            rsx! {
                HeroSelectionHotkeysView { loaded_keys, editing_section }
            }
        }
        SystemHotkeysCategory::ControlGroups => {
            rsx! {
                ControlGroupsHotkeysView { loaded_keys, editing_section }
            }
        }
        other_category => {
            rsx! {
                SystemHotkeysListView {
                    category: other_category,
                    loaded_keys,
                    editing_section,
                }
            }
        }
    }
}
