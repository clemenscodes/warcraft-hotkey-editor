use dioxus::prelude::*;
use warcraft_api::SystemKeybindModifier;
use warcraft_keybinds::{CustomKeys, SystemBindingMap};

use crate::components::system_hotkeys::key_cell::KeyCaptureCell;

#[component]
pub(super) fn SystemHotkeysListEntry(
    section_id: String,
    comment: String,
    default_hotkey: u32,
    default_modifier: SystemKeybindModifier,
    loaded_keys: Signal<Option<CustomKeys>>,
    editing_section: Signal<Option<String>>,
    binding_map: ReadSignal<SystemBindingMap>,
) -> Element {
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
                "{comment}"
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
