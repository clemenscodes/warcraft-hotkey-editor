pub mod components;
mod model;
mod view;

pub use view::SystemHotkeysListEntryView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::system_hotkeys_list_view::components::system_hotkeys_list_entry::components::key_capture::KeyCapture;
use components::system_hotkeys_list_entry_label::SystemHotkeysListEntryLabel;
use dioxus::prelude::*;
use model::SystemHotkeysListEntryModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SystemHotkeysListEntry(props: SystemHotkeysListEntryModel) -> Element {
    let text = props.comment.clone();
    let section_id = props.section_id;
    rsx! {
        li {
            class: CLASS,
            SystemHotkeysListEntryLabel {
                text,
            }
            KeyCapture {
                section_id,
            }
        }
    }
}

assert_component!(SystemHotkeysListEntry);
