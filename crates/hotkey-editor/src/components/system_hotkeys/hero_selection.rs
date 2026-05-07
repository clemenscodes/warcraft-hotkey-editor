use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

use crate::components::system_hotkeys::slot_button::SlotButton;
use crate::system_hotkeys::category::SystemHotkeysCategory;

const SLOT_FRAME_GOLD: Asset = asset!("/assets/webui/widgets/listitems/list-item-focus-border.png");

#[component]
pub(crate) fn HeroSelectionHotkeysView(
    loaded_keys: Signal<Option<CustomKeys>>,
    editing_section: Signal<Option<String>>,
) -> Element {
    let entries = SystemHotkeysCategory::HeroSelection.entries();
    let frame_url = SLOT_FRAME_GOLD;
    let frame_style = format!("--wc3-slot-frame: url('{frame_url}');");
    rsx! {
        div { class: "wc3-stage",
            p { class: "wc3-stage-hint",
                "Hotkeys for selecting your heroes by index."
            }
            div { class: "wc3-row wc3-row-3", style: "{frame_style}",
                for (slot_index, entry) in entries.iter().enumerate() {
                    SlotButton {
                        slot_label: format!("Hero {}", slot_index + 1),
                        section_id: entry.section_id().to_string(),
                        default_hotkey: entry.default_hotkey(),
                        default_modifier: entry.default_modifier(),
                        loaded_keys,
                        editing_section,
                    }
                }
            }
        }
    }
}
