mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::download_info_dialog::DownloadInfoDialog;
use dioxus::prelude::*;
use presentation::{ExportButtonPresentation, use_export_button};
use style::CLASS;
use tw_macro::assert_component;

/// The inline download action: the toolbar button that opens the download-info dialog, and that
/// dialog, mounted beneath it. It hides itself until a file is loaded, and owns the dialog's
/// open signal locally, so the dialog is part of the button and travels with it.
#[component]
pub fn ExportButton() -> Element {
    let ExportButtonPresentation {
        hidden,
        icon,
        aria_label,
        open,
        onclick,
        on_open_change,
    } = use_export_button();
    if hidden {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
            ToolbarButton {
                icon,
                aria_label,
                onclick,
            }
            DownloadInfoDialog {
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(ExportButton);
