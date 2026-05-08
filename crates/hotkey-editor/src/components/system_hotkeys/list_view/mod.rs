mod entry;

use dioxus::prelude::*;
use warcraft_database::SystemHotkeysCategory;
use warcraft_keybinds::{CustomKeys, SystemBindingMap};

use entry::SystemHotkeysListEntry;

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
        div { class: "flex flex-col items-center gap-8 w-full max-[1099px]:gap-[0.85rem]",
            ul { class: "list-none m-0 p-0 w-full max-w-[110rem] flex flex-col \
                         max-[1099px]:max-w-full max-[1099px]:[touch-action:pan-y]",
                for entry in entries {
                    {
                        let section_id = entry.section_id().to_string();
                        let comment = entry.comment().to_string();
                        let default_hotkey = entry.default_hotkey();
                        let default_modifier = entry.default_modifier();
                        rsx! {
                            SystemHotkeysListEntry {
                                key: "{section_id}",
                                section_id,
                                comment,
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
