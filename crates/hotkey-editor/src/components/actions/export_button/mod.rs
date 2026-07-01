mod hooks;
mod props;

use crate::components::dialogs::download_info_dialog::{
    DownloadInfoDialog, DownloadInfoDialogProps,
};
use crate::components::shared::toolbar_button::{ToolbarButton, ToolbarButtonProps};
use dioxus::prelude::*;
use hooks::use_export_button;
pub use props::ExportButtonProps;

/// Toolbar button that downloads the current `CustomKeys.txt`. Only present once
/// a file is loaded, since there is nothing to export otherwise.
#[component]
pub fn ExportButton(props: ExportButtonProps) -> Element {
    let model = use_export_button(&props);
    if !model.visible {
        return rsx! {};
    }
    let button = ToolbarButtonProps::from(&model);
    let dialog = DownloadInfoDialogProps::from(&model);
    rsx! {
        ToolbarButton { ..button }
        DownloadInfoDialog { ..dialog }
    }
}
