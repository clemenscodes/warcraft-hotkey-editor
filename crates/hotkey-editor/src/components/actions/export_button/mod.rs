use crate::components::dialogs::download_info_dialog::DownloadInfoDialog;
use crate::components::shared::icons::ICON_DOWNLOAD;
use crate::components::shared::toolbar_button::ToolbarButton;
use crate::services::files::download::BlobDownload;
use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

#[derive(Props, Clone, PartialEq)]
pub struct ExportButtonProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
}

/// Toolbar button that downloads the current `CustomKeys.txt`. Only present once
/// a file is loaded, since there is nothing to export otherwise.
#[component]
pub fn ExportButton(props: ExportButtonProps) -> Element {
    let loaded_keys = props.loaded_keys;
    let has_loaded_file = loaded_keys.read().is_some();
    let mut download_info_open = use_signal(|| false);
    let open_download_info = move |_| download_info_open.set(true);
    let handle_download_confirm = move |_| {
        let serialized = {
            let read_guard = loaded_keys.read();
            let Some(file) = read_guard.as_ref() else {
                return;
            };
            file.normalize().to_string()
        };
        BlobDownload::trigger("CustomKeys.txt", &serialized);
    };
    rsx! {
        if has_loaded_file {
            ToolbarButton {
                icon: ICON_DOWNLOAD,
                aria_label: "Download CustomKeys.txt",
                onclick: open_download_info,
            }
            DownloadInfoDialog { open: download_info_open, on_confirm: handle_download_confirm }
        }
    }
}
