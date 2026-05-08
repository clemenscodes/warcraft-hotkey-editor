use dioxus::prelude::*;
use warcraft_api::SystemKeybindModifier;
use warcraft_keybinds::{CustomKeys, SystemBindingMap};

use crate::components::system_hotkeys::key_cell::KeyCaptureCell;

#[derive(Props, Clone, PartialEq)]
pub(super) struct SystemHotkeysListEntryProps {
    pub(super) section_id: String,
    pub(super) comment: String,
    pub(super) default_hotkey: u32,
    pub(super) default_modifier: SystemKeybindModifier,
    pub(super) loaded_keys: Signal<Option<CustomKeys>>,
    pub(super) editing_section: Signal<Option<String>>,
    pub(super) binding_map: ReadSignal<SystemBindingMap>,
}

#[component]
pub(super) fn SystemHotkeysListEntry(props: SystemHotkeysListEntryProps) -> Element {
    let section_id = props.section_id;
    let comment = props.comment;
    let default_hotkey = props.default_hotkey;
    let default_modifier = props.default_modifier;
    let loaded_keys = props.loaded_keys;
    let editing_section = props.editing_section;
    let binding_map = props.binding_map;
    rsx! {
        li { class: "flex items-center justify-between gap-8 px-8 py-5 \
                     [border-top:1px_solid_rgba(255,206,99,0.14)] \
                     last:[border-bottom:1px_solid_rgba(255,206,99,0.14)] \
                     max-[1099px]:gap-[0.75rem] max-[1099px]:px-[0.5rem] \
                     max-[1099px]:py-[0.7rem] max-[1099px]:[touch-action:pan-y]",
            span {
                class: "font-friz-quadrata uppercase tracking-[0.08em] text-[2.8rem] leading-tight \
                        text-[#d6dcec] [text-shadow:1px_1px_0_#000] \
                        max-[1099px]:text-[clamp(12px,3.4vw,15px)] max-[1099px]:tracking-[0.04em] \
                        max-[1099px]:leading-[1.25] max-[1099px]:[flex:1_1_auto] max-[1099px]:min-w-0 \
                        max-[1099px]:[overflow-wrap:break-word] max-[1099px]:[word-break:break-word] \
                        max-[1099px]:whitespace-normal max-[360px]:text-[12px]",
                {comment}
            }
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
}
