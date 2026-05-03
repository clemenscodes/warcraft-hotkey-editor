use dioxus::prelude::*;
use warcraft_api::SystemKeybindModifier;
use warcraft_keybinds::CustomKeysFile;

use crate::components::system_hotkeys::key_picker_dialog::SystemKeyPickerDialog;
use crate::system_hotkeys::keycodes::{KeyCode, KeyCodes};

#[derive(Clone, Copy)]
pub(crate) struct EffectiveBinding {
    pub(crate) hotkey_code: u32,
    pub(crate) modifier: Option<&'static str>,
}

impl EffectiveBinding {
    pub(crate) fn resolve(
        loaded_keys: &Signal<Option<CustomKeysFile>>,
        section_id: &str,
        default_hotkey: u32,
        default_modifier: SystemKeybindModifier,
    ) -> Self {
        let read_guard = loaded_keys.read();
        let custom_hotkey = read_guard
            .as_ref()
            .and_then(|file| file.binding(section_id))
            .and_then(|binding| binding.hotkey())
            .and_then(|raw| raw.parse::<u32>().ok());
        let custom_modifier_owned = read_guard
            .as_ref()
            .and_then(|file| file.binding(section_id))
            .and_then(|binding| binding.modifier())
            .map(String::from);
        let hotkey_code = custom_hotkey.unwrap_or(default_hotkey);
        let modifier = match custom_modifier_owned.as_deref() {
            Some("Alt") => Some("Alt"),
            Some("Ctrl") => Some("Ctrl"),
            Some("Ctrl_or_Alt") => Some("Ctrl_or_Alt"),
            Some("Shift") => Some("Shift"),
            Some(_) | None => match default_modifier {
                SystemKeybindModifier::None => None,
                SystemKeybindModifier::Alt => Some("Alt"),
                SystemKeybindModifier::Ctrl => Some("Ctrl"),
                SystemKeybindModifier::CtrlOrAlt => Some("Ctrl_or_Alt"),
                SystemKeybindModifier::Shift => Some("Shift"),
            },
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
    let effective =
        EffectiveBinding::resolve(&loaded_keys, &lookup_id, default_hotkey, default_modifier);
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
            class: "wc3-key-chip",
            r#type: "button",
            onclick: move |_| editing_section.set(Some(section_id_for_click.clone())),
            "{key_label}"
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
