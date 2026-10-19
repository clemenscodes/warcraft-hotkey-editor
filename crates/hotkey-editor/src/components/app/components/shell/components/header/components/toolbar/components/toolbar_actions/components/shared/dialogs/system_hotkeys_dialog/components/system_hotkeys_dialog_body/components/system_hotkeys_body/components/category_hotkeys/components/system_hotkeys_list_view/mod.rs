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

#[component]
pub fn SystemHotkeysListView(props: SystemHotkeysListViewModel) -> Element {
    let category = props.category;
    let entries = category.entries();
    rsx! {
        ul {
            class: CLASS,
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
