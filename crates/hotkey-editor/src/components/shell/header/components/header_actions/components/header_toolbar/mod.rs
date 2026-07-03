pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::export_button_host::ExportButtonHost;
use components::help_button::HelpButton;
use components::preview_button::PreviewButton;
use components::redo_button::RedoButton;
use components::resolve_button_host::ResolveButtonHost;
use components::system_hotkeys_button::SystemHotkeysButton;
use components::templates_button::TemplatesButton;
use components::undo_button::UndoButton;
use components::upload_button::UploadButton;
use dioxus::prelude::*;
pub use props::HeaderToolbarProps;
use style::CLASS;
assert_component!(HeaderToolbar);

/// The inline file-action toolbar, shown only in the full header layout. In the
/// compact layout these actions live in the burger drawer instead.
#[component]
pub fn HeaderToolbar(props: HeaderToolbarProps) -> Element {
    let loaded_keys = props.loaded_keys;
    let upload_status = props.upload_status;
    rsx! {
        div { class: CLASS, role: "toolbar", aria_label: "File actions",
            UndoButton {}
            RedoButton {}
            UploadButton { loaded_keys, upload_status }
            TemplatesButton {}
            SystemHotkeysButton {}
            ResolveButtonHost {}
            PreviewButton {}
            ExportButtonHost {}
            HelpButton {}
        }
    }
}
