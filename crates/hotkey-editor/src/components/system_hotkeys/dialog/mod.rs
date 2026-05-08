mod breadcrumbs;
mod category_tab;
mod dialog_header;
mod inventory_drag_overlay;

use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use warcraft_keybinds::CustomKeys;

use crate::components::dialogs::dialog_stack::nested_picker_dialog_is_present;
use crate::components::system_hotkeys::control_groups::ControlGroupsHotkeysView;
use crate::components::system_hotkeys::hero_selection::HeroSelectionHotkeysView;
use crate::components::system_hotkeys::inventory::InventoryHotkeysView;
use crate::components::system_hotkeys::inventory_grid::InventoryDragFollower;
use crate::components::system_hotkeys::list_view::SystemHotkeysListView;
use warcraft_database::SystemHotkeysCategory;

use breadcrumbs::SystemHotkeysBreadcrumbs;
use dialog_header::SystemHotkeysHeader;
use inventory_drag_overlay::InventoryDragOverlay;

#[derive(Props, Clone, PartialEq)]
pub(crate) struct SystemHotkeysDialogProps {
    pub(crate) loaded_keys: Signal<Option<CustomKeys>>,
    pub(crate) system_hotkeys_open: Signal<bool>,
}

#[component]
pub(crate) fn SystemHotkeysDialog(props: SystemHotkeysDialogProps) -> Element {
    let loaded_keys = props.loaded_keys;
    let mut system_hotkeys_open = props.system_hotkeys_open;
    let editing_section = use_signal::<Option<String>>(|| None);
    let active_category = use_signal::<SystemHotkeysCategory>(|| SystemHotkeysCategory::Inventory);
    let drag_follower = use_signal::<Option<InventoryDragFollower>>(|| None);
    let active = *active_category.read();

    let handle_open_change = move |is_open: bool| {
        if !is_open && nested_picker_dialog_is_present() {
            return;
        }
        system_hotkeys_open.set(is_open);
    };
    let handle_close = move |_| system_hotkeys_open.set(false);
    rsx! {
        DialogRoot {
            class: "dialog-overlay",
            open: system_hotkeys_open(),
            on_open_change: handle_open_change,
            DialogContent { class: "dialog-shell wc3-dialog system-hotkeys-dialog".to_string(),
                SystemHotkeysHeader {
                    on_close: handle_close,
                }
                SystemHotkeysBreadcrumbs { active_category }
                div { class: "wc3-dialog-body",
                    match active {
                        SystemHotkeysCategory::Inventory => rsx! {
                            InventoryHotkeysView { loaded_keys, editing_section, drag_follower }
                        },
                        SystemHotkeysCategory::HeroSelection => rsx! {
                            HeroSelectionHotkeysView { loaded_keys, editing_section }
                        },
                        SystemHotkeysCategory::ControlGroups => rsx! {
                            ControlGroupsHotkeysView { loaded_keys, editing_section }
                        },
                        other_category => rsx! {
                            SystemHotkeysListView { category: other_category, loaded_keys, editing_section }
                        },
                    }
                }
                InventoryDragOverlay { drag_follower }
            }
        }
    }
}
