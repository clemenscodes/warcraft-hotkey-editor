mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::download_info_dialog::DownloadInfoDialog;
use dioxus::prelude::*;
use presentation::{ExportButtonPresentation, use_export_button};
use style::CLASS;
use tw_macro::assert_component;

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
