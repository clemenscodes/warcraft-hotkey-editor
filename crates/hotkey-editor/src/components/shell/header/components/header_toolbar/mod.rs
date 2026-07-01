mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

use crate::components::actions::export_button::ExportButton;
use crate::components::actions::help_button::HelpButton;
use crate::components::actions::preview_button::PreviewButton;
use crate::components::actions::redo_button::RedoButton;
use crate::components::actions::resolve_button::ResolveButton;
use crate::components::actions::system_hotkeys_button::SystemHotkeysButton;
use crate::components::actions::templates_button::TemplatesButton;
use crate::components::actions::undo_button::UndoButton;
use crate::components::actions::upload_button::UploadButton;

pub use props::HeaderToolbarProps;

assert_component!(HeaderToolbar);

/// The inline file-action toolbar, shown only in the full header layout. In the
/// compact layout these actions live in the burger drawer instead.
#[component]
pub fn HeaderToolbar(props: HeaderToolbarProps) -> Element {
    let loaded_keys = props.loaded_keys;
    let upload_status = props.upload_status;
    rsx! {
        div {
            class: CLASS,
            role: "toolbar",
            aria_label: "File actions",
            UndoButton {}
            RedoButton {}
            UploadButton { loaded_keys, upload_status }
            TemplatesButton {}
            SystemHotkeysButton {}
            ResolveButton { loaded_keys }
            PreviewButton {}
            ExportButton { loaded_keys }
            HelpButton {}
        }
    }
}
