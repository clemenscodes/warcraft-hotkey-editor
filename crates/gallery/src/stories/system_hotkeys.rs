use std::collections::HashMap;

use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::{
    ControlGroupsHotkeysView, HeroSelectionHotkeysView, InventoryCell, InventoryDragFollower,
    InventoryDragOverlay, InventoryDragSource, InventoryGrid, InventoryHotkeysView, KeyCaptureCell,
    SlotButton, SystemHotkeysBreadcrumbs, SystemHotkeysButton, SystemHotkeysCategoryTab,
    SystemHotkeysDialog, SystemHotkeysHeader, SystemHotkeysListEntry, SystemHotkeysListView,
    SystemKeyPickerDialog,
};
use warcraft_database::SystemHotkeysCategory;
use warcraft_keybinds::{CustomKeys, SystemBindingMap};

pub fn stories() -> Vec<Story> {
    vec![
        Story::new(
            "System hotkeys",
            "Inventory cell — default",
            inventory_cell_default,
        ),
        Story::new(
            "System hotkeys",
            "Key capture cell — default",
            key_capture_cell_default,
        ),
        Story::new(
            "System hotkeys",
            "List entry — general command",
            system_hotkeys_list_entry_default,
        ),
        Story::new(
            "System hotkeys",
            "Slot button — default",
            slot_button_default,
        ),
        Story::new("System hotkeys", "Button — closed", button_closed),
        Story::new("System hotkeys", "Button — open", button_open),
        Story::new("System hotkeys", "Header", header),
        Story::new(
            "System hotkeys",
            "Breadcrumbs — inventory selected",
            breadcrumbs_inventory,
        ),
        Story::new(
            "System hotkeys",
            "Category tab — active",
            category_tab_active,
        ),
        Story::new(
            "System hotkeys",
            "Category tab — inactive",
            category_tab_inactive,
        ),
        Story::new(
            "System hotkeys",
            "Inventory drag overlay — empty",
            inventory_drag_overlay_empty,
        ),
        Story::new("System hotkeys", "Control groups view", control_groups_view),
        Story::new("System hotkeys", "Hero selection view", hero_selection_view),
        Story::new("System hotkeys", "Inventory view", inventory_view),
        Story::new("System hotkeys", "Inventory grid", inventory_grid),
        Story::new(
            "System hotkeys",
            "Key picker dialog — open",
            key_picker_dialog_open,
        ),
        Story::new(
            "System hotkeys",
            "List view — general commands",
            list_view_general_commands,
        ),
        Story::new(
            "System hotkeys",
            "Dialog — open",
            system_hotkeys_dialog_open,
        ),
    ]
}

fn button_closed() -> Element {
    let system_hotkeys_open = use_signal(|| false);
    rsx! {
        SystemHotkeysButton { system_hotkeys_open }
    }
}

fn button_open() -> Element {
    let system_hotkeys_open = use_signal(|| true);
    rsx! {
        SystemHotkeysButton { system_hotkeys_open }
    }
}

fn header() -> Element {
    rsx! {
        SystemHotkeysHeader { on_close: move |_| {} }
    }
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
    let current_code: u32 = 65;
    let conflicts = HashMap::<u32, Vec<String>>::new();
    let open = true;
    rsx! {
        SystemKeyPickerDialog {
            title,
            current_code,
            conflicts,
            open,
            on_pick: move |_| {},
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
