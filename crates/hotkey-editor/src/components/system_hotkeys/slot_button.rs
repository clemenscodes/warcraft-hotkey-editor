use dioxus::prelude::*;
use warcraft_api::SystemKeybindModifier;
use warcraft_keybinds::{CustomKeysFile, Hotkey};

use crate::components::system_hotkeys::key_cell::EffectiveBinding;
use crate::components::system_hotkeys::key_picker_dialog::SystemKeyPickerDialog;
use crate::system_hotkeys::binding_map::SystemBindingMap;

/// Big WC3-style slot used in inventory-derived layouts (hero selection,
/// control groups). Same gold-frame visuals as the inventory cell minus the
/// pointer-event drag — those views aren't reorderable, just edit-on-click.
#[component]
pub(crate) fn SlotButton(
    slot_label: String,
    section_id: String,
    default_hotkey: u32,
    default_modifier: SystemKeybindModifier,
    mut loaded_keys: Signal<Option<CustomKeysFile>>,
    mut editing_section: Signal<Option<String>>,
) -> Element {
    let lookup_id = section_id.clone();
    let read_guard = loaded_keys.read();
    let custom_keys_ref = read_guard.as_ref();
    let effective = EffectiveBinding::resolve_from_file(
        custom_keys_ref,
        &lookup_id,
        default_hotkey,
        default_modifier,
    );
    let binding_map = SystemBindingMap::build(custom_keys_ref);
    drop(read_guard);
    let collisions =
        binding_map.collisions_for(&lookup_id, effective.hotkey_code, effective.modifier);
    let is_in_conflict = !collisions.is_empty();
    let conflict_title = if is_in_conflict {
        let names: Vec<String> = collisions
            .iter()
            .map(|resolved| resolved.section_comment().to_string())
            .collect();
        format!("Also used by {}", names.join(", "))
    } else {
        String::new()
    };
    let picker_conflicts = binding_map.picker_conflicts(&lookup_id, effective.modifier);
    let is_editing = editing_section
        .read()
        .as_deref()
        .map(|active| active == lookup_id.as_str())
        .unwrap_or(false);
    let key_label = if is_editing {
        String::from("…")
    } else {
        effective.label()
    };
    let mut cell_class = String::from("wc3-slot");
    if is_editing {
        cell_class.push_str(" editing");
    }
    if is_in_conflict {
        cell_class.push_str(" conflict");
    }
    let section_id_for_click = lookup_id.clone();
    let section_id_for_pick = lookup_id.clone();
    rsx! {
        button {
            class: "{cell_class}",
            r#type: "button",
            tabindex: "0",
            "data-tooltip": "{conflict_title}",
            onclick: move |_| editing_section.set(Some(section_id_for_click.clone())),
            div { class: "wc3-slot-label", "{slot_label}" }
            div { class: "wc3-slot-key", "{key_label}" }
        }
        if is_editing {
            SystemKeyPickerDialog {
                title: String::from("Pick a hotkey"),
                current_code: effective.hotkey_code,
                conflicts: picker_conflicts,
                open: true,
                on_pick: move |code: u32| {
                    let mut guard = loaded_keys.write();
                    let file = guard.get_or_insert_with(|| CustomKeysFile::from(""));
                    if let Some(binding) = file.system_mut(&section_id_for_pick) {
                        binding.set_hotkey(Hotkey::VirtualKey(code));
                    }
                    drop(guard);
                    editing_section.set(None);
                },
                on_close: move |_| editing_section.set(None),
            }
        }
    }
}
