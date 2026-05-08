use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

use crate::components::system_hotkeys::slot_button::SlotButton;
use crate::system_hotkeys::binding_map::SystemBindingMap;
use crate::system_hotkeys::category::SystemHotkeysCategory;

const SLOT_FRAME_GOLD: Asset = asset!("/assets/webui/widgets/listitems/list-item-focus-border.png");

#[component]
pub(crate) fn ControlGroupsHotkeysView(
    loaded_keys: Signal<Option<CustomKeys>>,
    editing_section: Signal<Option<String>>,
) -> Element {
    let entries = SystemHotkeysCategory::ControlGroups.entries();
    let binding_map = use_memo(move || {
        let guard = loaded_keys.read();
        SystemBindingMap::build(guard.as_ref())
    });
    let frame_url = SLOT_FRAME_GOLD;
    let frame_style = format!("--wc3-slot-frame: url('{frame_url}');");
    rsx! {
        div { class: "flex flex-col items-center gap-8 w-full max-[1099px]:gap-[0.85rem]",
            p { class: "m-0 text-[2rem] max-w-[90rem] text-center leading-snug font-friz-quadrata uppercase tracking-[0.1em] text-[rgba(255,206,99,0.75)] [text-shadow:1px_1px_0_#000] max-[1099px]:text-[clamp(11px,3vw,14px)] max-[1099px]:tracking-[0.04em] max-[1099px]:leading-[1.35] max-[1099px]:px-[0.25rem] max-[1099px]:max-w-full",
                "Hotkeys for control groups 1–10."
            }
            div { class: "wc3-row wc3-row-10", style: "{frame_style}",
                for (slot_index, entry) in entries.iter().enumerate() {
                    SlotButton {
                        slot_label: format!("{}", slot_index + 1),
                        section_id: entry.section_id().to_string(),
                        default_hotkey: entry.default_hotkey(),
                        default_modifier: entry.default_modifier(),
                        loaded_keys,
                        editing_section,
                        binding_map,
                    }
                }
            }
        }
    }
}
