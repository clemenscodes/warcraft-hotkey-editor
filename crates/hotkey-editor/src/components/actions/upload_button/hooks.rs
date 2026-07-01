use super::props::UploadButtonProps;
use crate::services::customkeys::upload_status::UploadStatus;
use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, use_toast};
use warcraft_keybinds::CustomKeys;

/// The upload button's shaped view: the info dialog signal and the two handlers.
pub(super) struct UploadButtonModel {
    pub(super) info_open: Signal<bool>,
    pub(super) on_file_change: EventHandler<FormEvent>,
    pub(super) on_open_info: EventHandler<MouseEvent>,
}

/// Reading the chosen file, normalizing it against the defaults, and reporting the
/// result via toast all live here so the body stays pure RSX.
pub(super) fn use_upload_button(props: &UploadButtonProps) -> UploadButtonModel {
    let mut loaded_keys = props.loaded_keys;
    let mut upload_status = props.upload_status;
    let toast_api = use_toast();
    let mut info_open = use_signal(|| false);
    let on_file_change = EventHandler::new(move |event: FormEvent| {
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
                        "{binding_count} ability bindings, {command_count} command bindings imported",
                    );
                    let options = ToastOptions::new().description(summary);
                    toast_api.success("CustomKeys.txt imported".to_string(), options);
                }
                Err(error) => {
                    let message = format!("{error}");
                    upload_status.set(UploadStatus::Error(message.clone()));
                    let options = ToastOptions::new().description(message);
                    toast_api.error("Import failed".to_string(), options);
                }
            }
        });
    });
    let on_open_info = EventHandler::new(move |_event: MouseEvent| info_open.set(true));
    UploadButtonModel {
        info_open,
        on_file_change,
        on_open_info,
    }
}
