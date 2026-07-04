mod hooks;
mod style;

use crate::assert_component;
use crate::components::dialogs::info_dialogs::download_info_dialog_host::DownloadInfoDialogHost;
use crate::components::shell::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use hooks::{use_export_button, ExportButtonPresentation};
use style::CLASS;

assert_component!(ExportButton);

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
