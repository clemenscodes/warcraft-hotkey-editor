use std::collections::HashMap;
use dioxus::prelude::*;
use dioxus_gallery::Story;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::SystemKeyPickerDialog;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::control_groups_hotkeys_view::ControlGroupsHotkeysView;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::SystemHotkeysDialog;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::inventory_drag_overlay::InventoryDragOverlay;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_breadcrumbs::SystemHotkeysBreadcrumbs;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_breadcrumbs::components::system_hotkeys_breadcrumbs_menu::components::system_hotkeys_category_tab::SystemHotkeysCategoryTab;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::hero_selection_hotkeys_view::HeroSelectionHotkeysView;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::InventoryHotkeysView;

use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::{
    InventoryDragFollower, InventoryDragSource, InventoryGrid,
};
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::components::inventory_slot::components::inventory_filled_slot::InventoryFilledSlot;

use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::system_hotkeys_list_view::components::system_hotkeys_list_entry::components::key_capture::KeyCapture;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::system_hotkeys_list_view::SystemHotkeysListView;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::system_hotkeys_list_view::components::system_hotkeys_list_entry::SystemHotkeysListEntry;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::slot_button::SlotButton;
use super::keys_mount::CustomKeysMount;
use warcraft_api::SystemHotkeysCategory;
use warcraft_keybinds::{CustomKeys, KeyCode, WarcraftObjectId};

pub fn stories() -> Vec<Story> {
    vec![
        Story::single(
            "System hotkeys",
            "InventoryFilledSlot",
            inventory_filled_slot_default,
        ),
        Story::single("System hotkeys", "KeyCapture", key_capture_default),
        Story::single(
            "System hotkeys",
            "SystemHotkeysListEntry",
            system_hotkeys_list_entry_default,
        ),
        Story::single("System hotkeys", "SlotButton", slot_button_default),
        Story::single(
            "System hotkeys",
            "SystemHotkeysBreadcrumbs",
            breadcrumbs_inventory,
        ),
        Story::new(
            "System hotkeys",
            "SystemHotkeysCategoryTab",
            "Active",
            category_tab_active,
        ),
        Story::new(
            "System hotkeys",
            "SystemHotkeysCategoryTab",
            "Inactive",
            category_tab_inactive,
        ),
        Story::single(
            "System hotkeys",
            "InventoryDragOverlay",
            inventory_drag_overlay_empty,
        ),
        Story::single(
            "System hotkeys",
            "ControlGroupsHotkeysView",
            control_groups_view,
        ),
        Story::single(
            "System hotkeys",
            "HeroSelectionHotkeysView",
            hero_selection_view,
        ),
        Story::single("System hotkeys", "InventoryHotkeysView", inventory_view),
        Story::single("System hotkeys", "InventoryGrid", inventory_grid),
        Story::single(
            "System hotkeys",
            "SystemKeyPickerDialog",
            key_picker_dialog_open,
        ),
        Story::single(
            "System hotkeys",
            "SystemHotkeysListView",
            list_view_general_commands,
        ),
        Story::single(
            "System hotkeys",
            "SystemHotkeysDialog",
            system_hotkeys_dialog_open,
        ),
    ]
}

fn breadcrumbs_inventory() -> Element {
    let active_category = use_signal(|| SystemHotkeysCategory::Inventory);
    rsx! {
        SystemHotkeysBreadcrumbs { active_category }
    }
}

fn category_tab_active() -> Element {
    let category = SystemHotkeysCategory::Inventory;
    let is_active = true;
    let has_separator = true;
    let active_category = use_signal(|| SystemHotkeysCategory::Inventory);
    let picker_open = use_signal(|| false);
    rsx! {
        SystemHotkeysCategoryTab {
            category,
            is_active,
            has_separator,
            active_category,
            picker_open,
        }
    }
}

fn category_tab_inactive() -> Element {
    let category = SystemHotkeysCategory::Camera;
    let is_active = false;
    let has_separator = false;
    let active_category = use_signal(|| SystemHotkeysCategory::Inventory);
    let picker_open = use_signal(|| false);
    rsx! {
        SystemHotkeysCategoryTab {
            category,
            is_active,
            has_separator,
            active_category,
            picker_open,
        }
    }
}

fn inventory_drag_overlay_empty() -> Element {
    let drag_follower = use_signal(|| None::<InventoryDragFollower>);
    rsx! {
        InventoryDragOverlay { drag_follower }
    }
}

fn control_groups_view() -> Element {
    let loaded_keys = use_signal(|| Some(CustomKeys::from_text("")));
    let editing_section = use_signal(|| None::<WarcraftObjectId>);
    rsx! {
        CustomKeysMount { loaded_keys,
            ControlGroupsHotkeysView { editing_section }
        }
    }
}

fn hero_selection_view() -> Element {
    let loaded_keys = use_signal(|| Some(CustomKeys::from_text("")));
    let editing_section = use_signal(|| None::<WarcraftObjectId>);
    rsx! {
        CustomKeysMount { loaded_keys,
            HeroSelectionHotkeysView { editing_section }
        }
    }
}

fn inventory_view() -> Element {
    let loaded_keys = use_signal(|| Some(CustomKeys::from_text("")));
    let editing_section = use_signal(|| None::<WarcraftObjectId>);
    let drag_follower = use_signal(|| None::<InventoryDragFollower>);
    rsx! {
        CustomKeysMount { loaded_keys,
            InventoryHotkeysView { editing_section, drag_follower }
        }
    }
}

fn inventory_grid() -> Element {
    let loaded_keys = use_signal(|| Some(CustomKeys::from_text("")));
    let editing_section = use_signal(|| None::<WarcraftObjectId>);
    let drag_follower = use_signal(|| None::<InventoryDragFollower>);
    rsx! {
        CustomKeysMount { loaded_keys,
            InventoryGrid { editing_section, drag_follower }
        }
    }
}

fn key_picker_dialog_open() -> Element {
    let title = "Pick a hotkey".to_string();
    let current_code = KeyCode::try_from(65).expect("valid keycode");
    let conflicts = HashMap::<KeyCode, Vec<String>>::new();
    let open = true;
    rsx! {
        SystemKeyPickerDialog {
            title,
            current_code,
            conflicts,
            open,
            on_pick: move | _
                    | {},
            on_close: move |_| {},
        }
    }
}

fn list_view_general_commands() -> Element {
    let category = SystemHotkeysCategory::GeneralCommands;
    let loaded_keys = use_signal(|| Some(CustomKeys::from_text("")));
    let editing_section = use_signal(|| None::<WarcraftObjectId>);
    rsx! {
        CustomKeysMount { loaded_keys,
            SystemHotkeysListView { category, editing_section }
        }
    }
}

fn system_hotkeys_dialog_open() -> Element {
    let loaded_keys = use_signal(|| Some(CustomKeys::from_text("")));
    let system_hotkeys_open = use_signal(|| true);
    rsx! {
        CustomKeysMount { loaded_keys,
            SystemHotkeysDialog { system_hotkeys_open }
        }
    }
}

fn inventory_filled_slot_default() -> Element {
    let entries = SystemHotkeysCategory::Inventory.entries();
    let first_entry = entries[0];
    let slot_index: usize = 0;
    let section_id = first_entry.section_id();
    let loaded_keys = use_signal(|| Some(CustomKeys::from_text("")));
    let editing_section = use_signal(|| None::<WarcraftObjectId>);
    let dragging_source = use_signal(|| None::<InventoryDragSource>);
    let drop_target = use_signal(|| None::<WarcraftObjectId>);
    let drag_follower = use_signal(|| None::<InventoryDragFollower>);
    rsx! {
        CustomKeysMount { loaded_keys,
            InventoryFilledSlot {
                slot_index,
                section_id,
                editing_section,
                dragging_source,
                drop_target,
                drag_follower,
            }
        }
    }
}

fn key_capture_default() -> Element {
    let entries = SystemHotkeysCategory::GeneralCommands.entries();
    let first_entry = entries[0];
    let section_id = first_entry.section_id();
    let loaded_keys = use_signal(|| Some(CustomKeys::from_text("")));
    let editing_section = use_signal(|| None::<WarcraftObjectId>);
    rsx! {
        CustomKeysMount { loaded_keys,
            KeyCapture {
                section_id,
                editing_section,
            }
        }
    }
}

fn system_hotkeys_list_entry_default() -> Element {
    let entries = SystemHotkeysCategory::GeneralCommands.entries();
    let first_entry = entries[0];
    let section_id = first_entry.section_id();
    let comment = first_entry.comment().to_string();
    let loaded_keys = use_signal(|| Some(CustomKeys::from_text("")));
    let editing_section = use_signal(|| None::<WarcraftObjectId>);
    rsx! {
        CustomKeysMount { loaded_keys,
            ul {
                SystemHotkeysListEntry {
                    section_id,
                    comment,
                    editing_section,
                }
            }
        }
    }
}

fn slot_button_default() -> Element {
    let entries = SystemHotkeysCategory::Inventory.entries();
    let first_entry = entries[0];
    let slot_label = "Slot 1".to_string();
    let section_id = first_entry.section_id();
    let loaded_keys = use_signal(|| Some(CustomKeys::from_text("")));
    let editing_section = use_signal(|| None::<WarcraftObjectId>);
    rsx! {
        CustomKeysMount { loaded_keys,
            SlotButton {
                slot_label,
                section_id,
                editing_section,
            }
        }
    }
}
