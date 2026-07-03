use crate::components::shared::icons::ICON_DOWNLOAD;
use crate::components::shell::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButtonProps;
use dioxus::prelude::*;

/// The export button in its two shapes: hidden until a file is loaded, and the visible
/// download control once one is. `on_open` opens the download info dialog; the download
/// itself is owned by `DownloadInfoDialogHost`. Supplied by `ExportButtonHost`.
#[derive(Props, Clone, PartialEq)]
pub struct ExportButtonProps {
    pub visible: bool,
    pub info_open: Signal<bool>,
    pub on_open: EventHandler<MouseEvent>,
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
