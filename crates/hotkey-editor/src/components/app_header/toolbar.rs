use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

use crate::components::export_buttons::ExportButtons;
use crate::components::icons::{ICON_COG, ICON_TEMPLATES};
use crate::components::upload_button::UploadButton;
use crate::customkeys::upload_status::UploadStatus;

#[component]
pub(crate) fn HeaderToolbar(
    loaded_keys: Signal<Option<CustomKeys>>,
    upload_status: Signal<UploadStatus>,
    preview_open: Signal<bool>,
    templates_dialog_open: Signal<bool>,
    mut system_hotkeys_open: Signal<bool>,
) -> Element {
    rsx! {
        div {
            class: "app-header-actions",
            role: "toolbar",
            aria_label: "File actions",
            UploadButton { loaded_keys, upload_status }
            button {
                class: "toolbar-icon-button",
                r#type: "button",
                aria_label: "Browse layout templates",
                aria_haspopup: "dialog",
                aria_expanded: "{templates_dialog_open()}",
                onclick: move |_| {
                    let next = !*templates_dialog_open.read();
                    templates_dialog_open.set(next);
                },
                span {
                    class: "toolbar-icon",
                    aria_hidden: "true",
                    dangerous_inner_html: ICON_TEMPLATES,
                }
            }
            button {
                class: if system_hotkeys_open() { "toolbar-icon-button active" } else { "toolbar-icon-button" },
                r#type: "button",
                aria_label: "General hotkeys",
                aria_haspopup: "dialog",
                aria_expanded: "{system_hotkeys_open()}",
                onclick: move |_| {
                    let next = !*system_hotkeys_open.read();
                    system_hotkeys_open.set(next);
                },
                span {
                    class: "toolbar-icon",
                    aria_hidden: "true",
                    dangerous_inner_html: ICON_COG,
                }
            }
            ExportButtons { loaded_keys, preview_open }
        }
    }
}
