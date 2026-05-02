use dioxus::prelude::*;
use warcraft_api::SystemKeybindModifier;
use warcraft_keybinds::CustomKeysFile;

use crate::components::system_hotkeys::key_cell::EffectiveBinding;
use crate::components::system_hotkeys::key_picker_dialog::SystemKeyPickerDialog;

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
    let effective =
        EffectiveBinding::resolve(&loaded_keys, &lookup_id, default_hotkey, default_modifier);
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
    let cell_class = if is_editing {
        "wc3-slot editing"
    } else {
        "wc3-slot"
    };
    let section_id_for_click = lookup_id.clone();
    let section_id_for_pick = lookup_id.clone();
    rsx! {
        button {
            class: "{cell_class}",
            r#type: "button",
            tabindex: "0",
            onclick: move |_| editing_section.set(Some(section_id_for_click.clone())),
            div { class: "wc3-slot-label", "{slot_label}" }
            div { class: "wc3-slot-key", "{key_label}" }
        }
        if is_editing {
            SystemKeyPickerDialog {
                title: String::from("Pick a hotkey"),
                current_code: effective.hotkey_code,
                open: true,
                on_pick: move |code: u32| {
                    let mut guard = loaded_keys.write();
                    let file = guard.get_or_insert_with(|| CustomKeysFile::from(""));
                    file.binding_or_default_mut(&section_id_for_pick)
                        .set_hotkey(Some(code.to_string()));
                    drop(guard);
                    editing_section.set(None);
                },
                on_close: move |_| editing_section.set(None),
            }
        }
    }
}
