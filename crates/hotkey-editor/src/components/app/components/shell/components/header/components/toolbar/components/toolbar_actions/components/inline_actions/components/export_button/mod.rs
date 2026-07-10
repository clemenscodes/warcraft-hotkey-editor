mod hooks;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::download_info_dialog_host::DownloadInfoDialogHost;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use hooks::{use_export_button, ExportButtonPresentation};
use style::CLASS;
use tw_macro::assert_component;

/// Toolbar button that downloads the current `CustomKeys.txt`, with its info dialog.
/// Reads the live document from context and hides itself until a file is loaded, since
/// there is nothing to export otherwise. Clicking opens the download info dialog; the
/// download itself is owned by `DownloadInfoDialogHost`.
#[component]
pub fn ExportButton() -> Element {
    let ExportButtonPresentation {
        visible,
        info_open: open,
        button,
    } = use_export_button();
    if !visible {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
            ToolbarButton { ..button }
            DownloadInfoDialogHost { open }
        }
    }
}

assert_component!(ExportButton);
