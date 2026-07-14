pub mod components;
mod style;

use components::export_button::ExportButton;
use components::help_button::HelpButton;
use components::preview_button::PreviewButton;
use components::redo_button::RedoButton;
use components::resolve_button::ResolveButton;
use components::system_hotkeys_button::SystemHotkeysButton;
use components::templates_button::TemplatesButton;
use components::undo_button::UndoButton;
use components::upload_button::UploadButton;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn InlineActions() -> Element {
    rsx! {
        div {
            class: CLASS,
            role: "toolbar",
            aria_label: "File actions",
            UndoButton {



            }
            RedoButton {



            }
            UploadButton {



            }
            TemplatesButton {



            }
            SystemHotkeysButton {



            }
            ResolveButton {



            }
            PreviewButton {



            }
            ExportButton {



            }
            HelpButton {



            }
        }
    }
}

assert_component!(InlineActions);
