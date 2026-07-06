pub mod components;
mod hooks;
mod props;
mod style;

use components::system_hotkeys_list_entry::SystemHotkeysListEntry;
use dioxus::prelude::*;
use hooks::use_system_hotkeys_list_view;
pub use props::SystemHotkeysListViewProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(SystemHotkeysListView);

/// A plain list editor: one row per hotkey in the category, each a binding name
/// beside its editable key chip.
#[component]
pub fn SystemHotkeysListView(props: SystemHotkeysListViewProps) -> Element {
    let category = props.category;
    let loaded_keys = props.loaded_keys;
    let editing_section = props.editing_section;
    let binding_map = use_system_hotkeys_list_view(&props);
    let entries = category.entries();
    rsx! {
        ul { class: CLASS,
            for entry in entries {
                SystemHotkeysListEntry {
                    key: "{entry.section_id()}",
                    section_id: entry.section_id().to_string(),
                    comment: entry.comment().to_string(),
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
