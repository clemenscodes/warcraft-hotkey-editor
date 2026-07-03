pub mod components;
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
use style::CLASS;
assert_component!(InlineActions);

/// The inline file-action buttons, shown at laptop width and up. Below that these
/// same actions collapse into the burger drawer that sits beside this row. Pure
/// layout — it threads no data; each button sources its own state.
#[component]
pub fn InlineActions() -> Element {
    rsx! {
        div { class: CLASS, role: "toolbar", aria_label: "File actions",
            UndoButton {}
            RedoButton {}
            UploadButton {}
            TemplatesButton {}
            SystemHotkeysButton {}
            ResolveButtonHost {}
            PreviewButton {}
            ExportButtonHost {}
            HelpButton {}
        }
    }
}
