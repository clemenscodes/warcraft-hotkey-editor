mod props;
mod style;

use crate::assert_component;
use crate::components::dialogs::info_dialogs::download_info_dialog_host::DownloadInfoDialogHost;
use crate::components::shell::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::{
    ToolbarButton, ToolbarButtonProps,
};
use dioxus::prelude::*;
pub use props::ExportButtonProps;
use style::CLASS;

assert_component!(ExportButton);

/// Toolbar button that downloads the current `CustomKeys.txt`, with its info dialog.
/// Presentational: it renders from `visible`, the dialog's open signal, and the open
/// handler alone. `ExportButtonHost` supplies those; the download itself is owned by
/// `DownloadInfoDialogHost`. Hidden until a file is loaded, since there is nothing to
/// export otherwise.
#[component]
pub fn ExportButton(props: ExportButtonProps) -> Element {
    if !props.visible {
        return rsx! {};
    }
    let button = ToolbarButtonProps::from(&props);
    let open = props.info_open;
    rsx! {
        div {
            class: CLASS,
            ToolbarButton { ..button }
            DownloadInfoDialogHost { open }
        }
    }
}
