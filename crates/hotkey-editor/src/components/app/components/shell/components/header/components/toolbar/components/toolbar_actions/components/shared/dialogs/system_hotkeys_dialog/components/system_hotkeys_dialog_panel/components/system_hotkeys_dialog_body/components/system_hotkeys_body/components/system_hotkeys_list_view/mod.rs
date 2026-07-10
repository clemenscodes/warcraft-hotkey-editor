pub mod components;
mod props;
mod style;

use components::system_hotkeys_list_entry::SystemHotkeysListEntry;
use dioxus::prelude::*;
pub use props::SystemHotkeysListViewProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(SystemHotkeysListView);

/// A plain list editor: one row per hotkey in the category, each a binding name
/// beside its editable key chip.
#[component]
pub fn SystemHotkeysListView(props: SystemHotkeysListViewProps) -> Element {
    let category = props.category;
    let editing_section = props.editing_section;
    let entries = category.entries();
    rsx! {
        ul { class: CLASS,
            for entry in entries {
                SystemHotkeysListEntry {
                    key: "{entry.section_id()}",
                    section_id: entry.section_id(),
                    comment: entry.comment().to_string(),
                    editing_section,
                }
            }
        }
    }
}
