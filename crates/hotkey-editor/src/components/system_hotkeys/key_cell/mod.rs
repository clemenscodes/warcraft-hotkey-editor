use dioxus::prelude::*;
use warcraft_api::SystemKeybindModifier;
use warcraft_keybinds::{CustomKeys, EffectiveBinding, Hotkey, SystemBindingMap};

use crate::components::system_hotkeys::key_picker_dialog::SystemKeyPickerDialog;

#[derive(Props, Clone, PartialEq)]
pub(crate) struct KeyCaptureCellProps {
    pub(crate) section_id: String,
    pub(crate) default_hotkey: u32,
    pub(crate) default_modifier: SystemKeybindModifier,
    pub(crate) loaded_keys: Signal<Option<CustomKeys>>,
    pub(crate) editing_section: Signal<Option<String>>,
    pub(crate) binding_map: ReadSignal<SystemBindingMap>,
}

#[component]
pub(crate) fn KeyCaptureCell(props: KeyCaptureCellProps) -> Element {
    let section_id = props.section_id;
    let default_hotkey = props.default_hotkey;
    let default_modifier = props.default_modifier;
    let mut loaded_keys = props.loaded_keys;
    let mut editing_section = props.editing_section;
    let binding_map = props.binding_map;
    let lookup_id = section_id.clone();
    let read_guard = loaded_keys.read();
    let custom_keys_ref = read_guard.as_ref();
    let effective = EffectiveBinding::resolve_from_file(
        custom_keys_ref,
        &lookup_id,
        default_hotkey,
        default_modifier,
    );
    drop(read_guard);
    let map_guard = binding_map.read();
    let collisions =
        map_guard.collisions_for(&lookup_id, effective.hotkey_code(), effective.modifier());
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
    const CHIP_BASE: &str = "system-key-cell inline-flex items-center justify-center \
        font-friz-quadrata uppercase tracking-[0.04em] text-[2.4rem] leading-none \
        px-6 py-3.5 cursor-pointer whitespace-nowrap \
        min-w-[18rem] \
        bg-[linear-gradient(180deg,rgba(15,22,45,0.85)_0%,rgba(8,14,30,0.95)_100%)] \
        border rounded-[2px] [transition:filter_0.15s_ease,border-color_0.15s_ease] \
        hover:[filter:brightness(1.18)_drop-shadow(0_0_8px_rgba(255,206,99,0.4))] \
        hover:border-[rgba(255,206,99,0.85)] \
        [body[data-kb-modality]_&]:focus:outline-none \
        [body[data-kb-modality]_&]:focus:border-white \
        [body[data-kb-modality]_&]:focus:[filter:drop-shadow(0_0_10px_rgba(255,255,255,0.55))] \
        max-[1099px]:min-w-[5.5rem] max-[1099px]:max-w-[14rem] \
        max-[1099px]:text-[clamp(13px,3.6vw,16px)] max-[1099px]:px-[0.7rem] max-[1099px]:py-[0.5rem] \
        max-[1099px]:[flex:0_0_auto] max-[1099px]:overflow-hidden max-[1099px]:text-ellipsis \
        max-[1099px]:[touch-action:manipulation] \
        max-[360px]:min-w-[4.5rem] max-[360px]:text-[12px] max-[360px]:px-[0.55rem] max-[360px]:py-[0.45rem]";
    const CHIP_NORMAL: &str =
        "text-[#ffce63] [text-shadow:1px_1px_0_#000] border-[rgba(255,206,99,0.45)]";
    const CHIP_CONFLICT: &str = "text-[#ff5a5a] [text-shadow:1px_1px_0_#000,0_0_10px_rgba(255,90,90,0.5)] border-[rgba(255,90,90,0.65)]";
    let chip_class = if is_in_conflict {
        format!("{CHIP_BASE} {CHIP_CONFLICT}")
    } else {
        format!("{CHIP_BASE} {CHIP_NORMAL}")
    };
    let picker_conflicts = map_guard.picker_conflicts(&lookup_id, effective.modifier());
    drop(map_guard);
    let is_editing = editing_section
        .read()
        .as_deref()
        .map(|active| active == lookup_id.as_str())
        .unwrap_or(false);
    let key_label = effective.label();
    let section_id_for_click = lookup_id.clone();
    let section_id_for_pick = lookup_id.clone();
    let handle_click = move |_| editing_section.set(Some(section_id_for_click.clone()));
    let handle_pick = move |code: u32| {
        let mut guard = loaded_keys.write();
        let file = guard.get_or_insert_with(|| CustomKeys::from(""));
        if let Some(binding) = file.system_mut(&section_id_for_pick) {
            binding.set_hotkey(Hotkey::VirtualKey(code));
        }
        drop(guard);
        editing_section.set(None);
    };
    let handle_picker_close = move |_| editing_section.set(None);
    rsx! {
        button {
            class: chip_class,
            r#type: "button",
            "data-tooltip": conflict_title,
            "data-tooltip-placement": "above",
            onclick: handle_click,
            {key_label}
        }
        if is_editing {
            SystemKeyPickerDialog {
                title: String::from("Pick a hotkey"),
                current_code: effective.hotkey_code(),
                conflicts: picker_conflicts,
                open: true,
                on_pick: handle_pick,
                on_close: handle_picker_close,
            }
        }
    }
}
