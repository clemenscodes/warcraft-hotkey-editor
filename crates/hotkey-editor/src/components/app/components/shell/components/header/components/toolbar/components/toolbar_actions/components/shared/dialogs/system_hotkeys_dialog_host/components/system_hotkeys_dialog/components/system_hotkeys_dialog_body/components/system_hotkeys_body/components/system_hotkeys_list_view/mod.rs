pub mod components;
mod model;
mod view;

pub use view::SystemHotkeysListViewView;
mod style;

use components::system_hotkeys_list_entry::SystemHotkeysListEntry;
use dioxus::prelude::*;
use model::SystemHotkeysListViewModel;
use style::CLASS;
use tw_macro::assert_component;

/// A plain list editor: one row per hotkey in the category, each a binding name
/// beside its editable key chip.
#[component]
pub fn SystemHotkeysListView(props: SystemHotkeysListViewModel) -> Element {
    let category = props.category;
    let entries = category.entries();
    rsx! {
        ul { class: CLASS,
            for entry in entries {
                SystemHotkeysListEntry {
                    key: "{entry.section_id()}",
                    section_id: entry.section_id(),
                    comment: entry.comment().to_string(),
                }
            }
        }
    }
}

assert_component!(SystemHotkeysListView);
