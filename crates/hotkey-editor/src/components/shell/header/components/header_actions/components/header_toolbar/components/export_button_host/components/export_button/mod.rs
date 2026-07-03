mod props;
mod style;

use crate::assert_component;
use crate::components::dialogs::info_dialogs::download_info_dialog::{
    DownloadInfoDialog, DownloadInfoDialogProps,
};
use crate::components::shell::header::components::header_actions::components::header_toolbar::components::shared::toolbar_button::{
    ToolbarButton, ToolbarButtonProps,
};
use dioxus::prelude::*;
pub use props::ExportButtonProps;
use style::CLASS;

assert_component!(ExportButton);

/// Toolbar button that downloads the current `CustomKeys.txt`, with its info dialog.
/// Presentational: it renders from `visible`, the dialog signal, and the two handlers
/// alone and fetches nothing, so the gallery can showcase it with plain values.
/// `ExportButtonHost` supplies them from context. Hidden until a file is loaded, since
/// there is nothing to export otherwise.
#[component]
pub fn ExportButton(props: ExportButtonProps) -> Element {
    if !props.visible {
        return rsx! {};
    }
    let button = ToolbarButtonProps::from(&props);
    let dialog = DownloadInfoDialogProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            ToolbarButton { ..button }
            DownloadInfoDialog { ..dialog }
        }
    }
}
