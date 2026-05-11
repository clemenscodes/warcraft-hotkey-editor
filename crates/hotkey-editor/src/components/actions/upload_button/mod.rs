use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, use_toast};
use warcraft_keybinds::CustomKeys;

use crate::components::dialogs::upload_info_dialog::UploadInfoDialog;
use crate::components::shared::icons::ICON_UPLOAD;
use crate::components::shell::header::{TOOLBAR_BTN_CLASS, TOOLBAR_ICON_CLASS};

use crate::services::customkeys::upload_status::UploadStatus;
use crate::services::files::upload::UPLOAD_INPUT_ELEMENT_ID;

#[derive(Props, Clone, PartialEq)]
pub(crate) struct UploadButtonProps {
    pub(crate) loaded_keys: Signal<Option<CustomKeys>>,
    pub(crate) upload_status: Signal<UploadStatus>,
}

#[component]
pub(crate) fn UploadButton(props: UploadButtonProps) -> Element {
    let mut loaded_keys = props.loaded_keys;
    let mut upload_status = props.upload_status;
    let toast_api = use_toast();
    let mut info_open = use_signal(|| false);
    let on_file_change = move |event: Event<FormData>| {
        let files = event.files();
        let Some(first_file) = files.into_iter().next() else {
            upload_status.set(UploadStatus::Error("No file selected".into()));
            toast_api.error("No file selected".to_string(), ToastOptions::new());
            return;
        };
        upload_status.set(UploadStatus::Loading);
        spawn(async move {
            match first_file.read_string().await {
                Ok(contents) => {
                    let uploaded_only = CustomKeys::from(contents.as_str());
                    let binding_count = uploaded_only.bindings_in_order().count();
                    let command_count = uploaded_only.commands_in_order().count();
                    let mut baseline_file =
                        CustomKeys::from(warcraft_keybinds::DEFAULT_CUSTOM_KEYS);
                    baseline_file.extend(uploaded_only);
                    let normalized = baseline_file.normalize();
                    loaded_keys.set(Some(normalized));
                    let loaded_status = UploadStatus::Loaded {
                        binding_count,
                        command_count,
                    };
                    upload_status.set(loaded_status);
                    let summary = format!(
                        "{binding_count} ability bindings, {command_count} command bindings imported"
                    );
                    toast_api.success(
                        "CustomKeys.txt imported".to_string(),
                        ToastOptions::new().description(summary),
                    );
                }
                Err(error) => {
                    let message = format!("{error}");
                    upload_status.set(UploadStatus::Error(message.clone()));
                    toast_api.error(
                        "Import failed".to_string(),
                        ToastOptions::new().description(message),
                    );
                }
            }
        });
    };

    let open_info = move |_| info_open.set(true);
    rsx! {
        div { class: "contents",
            input {
                id: UPLOAD_INPUT_ELEMENT_ID,
                class: "absolute -left-[9999px] w-px h-px opacity-0",
                r#type: "file",
                accept: ".txt,text/plain",
                onchange: on_file_change,
            }
            button {
                class: TOOLBAR_BTN_CLASS,
                r#type: "button",
                aria_label: "Upload CustomKeys.txt",
                onclick: open_info,
                span {
                    class: TOOLBAR_ICON_CLASS,
                    aria_hidden: "true",
                    dangerous_inner_html: ICON_UPLOAD,
                }
            }
            UploadInfoDialog { open: info_open }
        }
    }
}
