use dioxus::prelude::*;
use warcraft_api::SystemKeybindModifier;
use warcraft_keybinds::CustomKeysFile;

use crate::components::system_hotkeys::key_picker_dialog::SystemKeyPickerDialog;
use crate::system_hotkeys::binding_map::SystemBindingMap;
use crate::system_hotkeys::keycodes::{KeyCode, KeyCodes};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct EffectiveBinding {
    pub(crate) hotkey_code: u32,
    pub(crate) modifier: Option<&'static str>,
}

impl EffectiveBinding {
    pub(crate) fn resolve_from_file(
        custom_keys: Option<&CustomKeysFile>,
        section_id: &str,
        default_hotkey: u32,
        default_modifier: SystemKeybindModifier,
    ) -> Self {
        let custom_hotkey = custom_keys
            .and_then(|file| file.binding(section_id))
            .and_then(|binding| binding.hotkey())
            .and_then(|raw| raw.parse::<u32>().ok());
        let hotkey_code = custom_hotkey.unwrap_or(default_hotkey);
        // Warcraft III hardcodes the modifier per system hotkey — any
        // `Modifier=` line in CustomKeys.txt is written for transparency but
        // discarded at load time. The editor mirrors that: the effective
        // modifier is always the system default, regardless of the file.
        let modifier = match default_modifier {
            SystemKeybindModifier::None => None,
            SystemKeybindModifier::Alt => Some("Alt"),
            SystemKeybindModifier::Ctrl => Some("Ctrl"),
            SystemKeybindModifier::CtrlOrAlt => Some("Ctrl_or_Alt"),
            SystemKeybindModifier::Shift => Some("Shift"),
        };
        Self {
            hotkey_code,
            modifier,
        }
    }

    pub(crate) fn label(&self) -> String {
        let modifier_text = KeyCodes::modifier_prefix(self.modifier);
        let code = KeyCode::from(self.hotkey_code);
        format!("{modifier_text}{code}")
    }
}

#[component]
pub(crate) fn KeyCaptureCell(
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
    let collisions = binding_map.collisions_for(
        &lookup_id,
        effective.hotkey_code,
        effective.modifier,
    );
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
    let chip_class = if is_in_conflict {
        "wc3-key-chip conflict"
    } else {
        "wc3-key-chip"
    };
    let picker_conflicts = binding_map.picker_conflicts(&lookup_id, effective.modifier);
    let is_editing = editing_section
        .read()
        .as_deref()
        .map(|active| active == lookup_id.as_str())
        .unwrap_or(false);
    let key_label = effective.label();
    let section_id_for_click = lookup_id.clone();
    let section_id_for_pick = lookup_id.clone();
    rsx! {
        button {
            class: "{chip_class}",
            r#type: "button",
            "data-tooltip": "{conflict_title}",
            onclick: move |_| editing_section.set(Some(section_id_for_click.clone())),
            "{key_label}"
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
