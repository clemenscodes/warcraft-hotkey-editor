use std::collections::HashMap;
use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::components::dialogs::system_key_picker_dialog::SystemKeyPickerDialog;
use hotkey_editor::components::dialogs::system_hotkeys_dialog::components::control_groups_hotkeys_view::ControlGroupsHotkeysView;
use hotkey_editor::components::dialogs::system_hotkeys_dialog::SystemHotkeysDialog;
use hotkey_editor::components::dialogs::system_hotkeys_dialog::components::inventory_drag_overlay::InventoryDragOverlay;
use hotkey_editor::components::dialogs::system_hotkeys_dialog::components::system_hotkeys_breadcrumbs::SystemHotkeysBreadcrumbs;
use hotkey_editor::components::dialogs::system_hotkeys_dialog::components::system_hotkeys_breadcrumbs::components::system_hotkeys_breadcrumbs_menu::components::system_hotkeys_category_tab::SystemHotkeysCategoryTab;
use hotkey_editor::components::dialogs::system_hotkeys_dialog::components::hero_selection_hotkeys_view::HeroSelectionHotkeysView;
use hotkey_editor::components::dialogs::system_hotkeys_dialog::components::inventory_hotkeys_view::InventoryHotkeysView;

use hotkey_editor::components::dialogs::system_hotkeys_dialog::components::inventory_grid::{
    InventoryCell, InventoryDragFollower, InventoryDragSource, InventoryGrid,
};

use hotkey_editor::components::dialogs::system_hotkeys_dialog::components::key_capture_cell::KeyCaptureCell;
use hotkey_editor::components::dialogs::system_hotkeys_dialog::components::system_hotkeys_list_view::SystemHotkeysListView;
use hotkey_editor::components::dialogs::system_hotkeys_dialog::components::system_hotkeys_list_view::components::system_hotkeys_list_entry::SystemHotkeysListEntry;
use hotkey_editor::components::dialogs::system_hotkeys_dialog::components::slot_button::SlotButton;
use warcraft_database::SystemHotkeysCategory;
use warcraft_keybinds::{CustomKeys, KeyCode, SystemBindingMap};

pub fn stories() -> Vec<Story> {
    vec![
        Story::single("System hotkeys", "InventoryCell", inventory_cell_default),
        Story::single("System hotkeys", "KeyCaptureCell", key_capture_cell_default),
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
    let loaded_keys = use_signal(|| Some(CustomKeys::from("").normalize()));
    let editing_section = use_signal(|| None::<String>);
    rsx! {
        ControlGroupsHotkeysView { loaded_keys, editing_section }
    }
}

fn hero_selection_view() -> Element {
    let loaded_keys = use_signal(|| Some(CustomKeys::from("").normalize()));
    let editing_section = use_signal(|| None::<String>);
    rsx! {
        HeroSelectionHotkeysView { loaded_keys, editing_section }
    }
}

fn inventory_view() -> Element {
    let loaded_keys = use_signal(|| Some(CustomKeys::from("").normalize()));
    let editing_section = use_signal(|| None::<String>);
    let drag_follower = use_signal(|| None::<InventoryDragFollower>);
    rsx! {
        InventoryHotkeysView { loaded_keys, editing_section, drag_follower }
    }
}

fn inventory_grid() -> Element {
    let loaded_keys = use_signal(|| Some(CustomKeys::from("").normalize()));
    let editing_section = use_signal(|| None::<String>);
    let drag_follower = use_signal(|| None::<InventoryDragFollower>);
    rsx! {
        InventoryGrid { loaded_keys, editing_section, drag_follower }
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
    let loaded_keys = use_signal(|| Some(CustomKeys::from("").normalize()));
    let editing_section = use_signal(|| None::<String>);
    rsx! {
        SystemHotkeysListView { category, loaded_keys, editing_section }
    }
}

fn system_hotkeys_dialog_open() -> Element {
    let loaded_keys = use_signal(|| Some(CustomKeys::from("").normalize()));
    let system_hotkeys_open = use_signal(|| true);
    rsx! {
        SystemHotkeysDialog { loaded_keys, system_hotkeys_open }
    }
}

fn inventory_cell_default() -> Element {
    let entries = SystemHotkeysCategory::Inventory.entries();
    let first_entry = entries[0];
    let slot_index: usize = 0;
    let section_id = first_entry.section_id().to_string();
    let default_hotkey = first_entry.default_hotkey();
    let default_modifier = first_entry.default_modifier();
    let loaded_keys = use_signal(|| Some(CustomKeys::from("").normalize()));
    let editing_section = use_signal(|| None::<String>);
    let dragging_source = use_signal(|| None::<InventoryDragSource>);
    let drop_target = use_signal(|| None::<String>);
    let drag_follower = use_signal(|| None::<InventoryDragFollower>);
    let binding_map = use_memo(move || {
        let guard = loaded_keys.read();
        SystemBindingMap::build(guard.as_ref())
    });
    rsx! {
        InventoryCell {
            slot_index,
            section_id,
            default_hotkey,
            default_modifier,
            loaded_keys,
            editing_section,
            dragging_source,
            drop_target,
            drag_follower,
            binding_map,
        }
    }
}

fn key_capture_cell_default() -> Element {
    let entries = SystemHotkeysCategory::GeneralCommands.entries();
    let first_entry = entries[0];
    let section_id = first_entry.section_id().to_string();
    let default_hotkey = first_entry.default_hotkey();
    let default_modifier = first_entry.default_modifier();
    let loaded_keys = use_signal(|| Some(CustomKeys::from("").normalize()));
    let editing_section = use_signal(|| None::<String>);
    let binding_map = use_memo(move || {
        let guard = loaded_keys.read();
        SystemBindingMap::build(guard.as_ref())
    });
    rsx! {
        KeyCaptureCell {
            section_id,
            default_hotkey,
            default_modifier,
            loaded_keys,
            editing_section,
            binding_map,
        }
    }
}

fn system_hotkeys_list_entry_default() -> Element {
    let entries = SystemHotkeysCategory::GeneralCommands.entries();
    let first_entry = entries[0];
    let section_id = first_entry.section_id().to_string();
    let comment = first_entry.comment().to_string();
    let default_hotkey = first_entry.default_hotkey();
    let default_modifier = first_entry.default_modifier();
    let loaded_keys = use_signal(|| Some(CustomKeys::from("").normalize()));
    let editing_section = use_signal(|| None::<String>);
    let binding_map = use_memo(move || {
        let guard = loaded_keys.read();
        SystemBindingMap::build(guard.as_ref())
    });
    rsx! {
        ul {
            SystemHotkeysListEntry {
                section_id,
                comment,
                default_hotkey,
                default_modifier,
                loaded_keys,
                editing_section,
                binding_map,
            }
        }
    }
}

fn slot_button_default() -> Element {
    let entries = SystemHotkeysCategory::Inventory.entries();
    let first_entry = entries[0];
    let slot_label = "Slot 1".to_string();
    let section_id = first_entry.section_id().to_string();
    let default_hotkey = first_entry.default_hotkey();
    let default_modifier = first_entry.default_modifier();
    let loaded_keys = use_signal(|| Some(CustomKeys::from("").normalize()));
    let editing_section = use_signal(|| None::<String>);
    let binding_map = use_memo(move || {
        let guard = loaded_keys.read();
        SystemBindingMap::build(guard.as_ref())
    });
    rsx! {
        SlotButton {
            slot_label,
            section_id,
            default_hotkey,
            default_modifier,
            loaded_keys,
            editing_section,
            binding_map,
        }
    }
}
