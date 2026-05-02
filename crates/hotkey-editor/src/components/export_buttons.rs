use dioxus::prelude::*;
use warcraft_keybinds::CustomKeysFile;

use crate::components::download_info_dialog::DownloadInfoDialog;
use crate::components::icons::{ICON_DOWNLOAD, ICON_PREVIEW};
use crate::customkeys::blob_download::BlobDownload;
use crate::customkeys::explicit_export::ExplicitExport;

#[component]
pub(crate) fn ExportButtons(
    loaded_keys: Signal<Option<CustomKeysFile>>,
    mut preview_open: Signal<bool>,
) -> Element {
    let has_loaded_file = loaded_keys.read().is_some();
    let preview_visible = *preview_open.read();
    let preview_label = if preview_visible {
        "Hide preview"
    } else {
        "Preview"
    };
    let mut download_info_open = use_signal(|| false);

    rsx! {
        div { class: "contents",
            button {
                class: "toolbar-icon-button",
                r#type: "button",
                aria_label: "{preview_label}",
                aria_pressed: "{preview_visible}",
                onclick: move |_| {
                    let next_value = !*preview_open.read();
                    preview_open.set(next_value);
                },
                span {
                    class: "toolbar-icon",
                    aria_hidden: "true",
                    dangerous_inner_html: ICON_PREVIEW,
                }
            }
            if has_loaded_file {
                button {
                    class: "toolbar-icon-button",
                    r#type: "button",
                    aria_label: "Download CustomKeys.txt",
                    onclick: move |_| download_info_open.set(true),
                    span {
                        class: "toolbar-icon",
                        dangerous_inner_html: ICON_DOWNLOAD,
                    }
                }
                DownloadInfoDialog {
                    open: download_info_open,
                    on_confirm: move |_| {
                        let serialized = {
                            let read_guard = loaded_keys.read();
                            let Some(file) = read_guard.as_ref() else { return };
                            ExplicitExport::serialize(file)
                        };
                        BlobDownload::trigger("CustomKeys.txt", &serialized);
                    },
                }
            }
        }
    }
}
