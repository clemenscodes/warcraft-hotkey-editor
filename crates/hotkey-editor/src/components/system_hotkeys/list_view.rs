use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

use crate::components::system_hotkeys::key_cell::KeyCaptureCell;
use crate::system_hotkeys::binding_map::SystemBindingMap;
use crate::system_hotkeys::category::SystemHotkeysCategory;

#[component]
pub(crate) fn SystemHotkeysListView(
    category: SystemHotkeysCategory,
    loaded_keys: Signal<Option<CustomKeys>>,
    editing_section: Signal<Option<String>>,
) -> Element {
    let entries = category.entries();
    let binding_map = use_memo(move || {
        let guard = loaded_keys.read();
        SystemBindingMap::build(guard.as_ref())
    });
    rsx! {
        div { class: "wc3-stage",
            ul { class: "wc3-binding-list",
                for entry in entries {
                    {
                        let section_id = entry.section_id().to_string();
                        let comment = entry.comment().to_string();
                        let default_hotkey = entry.default_hotkey();
                        let default_modifier = entry.default_modifier();
                        rsx! {
                            li { class: "wc3-binding-row",
                                span { class: "wc3-binding-name", "{comment}" }
                                KeyCaptureCell {
                                    section_id: section_id.clone(),
                                    default_hotkey,
                                    default_modifier,
                                    loaded_keys,
                                    editing_section,
                                    binding_map,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
