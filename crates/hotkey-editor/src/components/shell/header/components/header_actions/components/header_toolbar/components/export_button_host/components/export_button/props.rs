use crate::components::dialogs::info_dialogs::download_info_dialog::DownloadInfoDialogProps;
use crate::components::shared::icons::ICON_DOWNLOAD;
use crate::components::shell::header::components::header_actions::components::header_toolbar::components::shared::toolbar_button::ToolbarButtonProps;
use dioxus::prelude::*;

/// The export button in its two shapes: hidden until a file is loaded, and the visible
/// download control once one is. `on_open` opens the info dialog; `on_confirm` triggers
/// the download. All are supplied by `ExportButtonHost`; the leaf itself fetches nothing.
#[derive(Props, Clone, PartialEq)]
pub struct ExportButtonProps {
    pub visible: bool,
    pub info_open: Signal<bool>,
    pub on_open: EventHandler<MouseEvent>,
    pub on_confirm: EventHandler<()>,
}

impl From<&ExportButtonProps> for ToolbarButtonProps {
    fn from(props: &ExportButtonProps) -> Self {
        let onclick = props.on_open;
        Self {
            icon: ICON_DOWNLOAD,
            aria_label: "Download CustomKeys.txt",
            onclick,
            ..Self::default()
        }
    }
}

impl From<&ExportButtonProps> for DownloadInfoDialogProps {
    fn from(props: &ExportButtonProps) -> Self {
        let open = props.info_open;
        let on_confirm = props.on_confirm;
        Self { open, on_confirm }
    }
}
