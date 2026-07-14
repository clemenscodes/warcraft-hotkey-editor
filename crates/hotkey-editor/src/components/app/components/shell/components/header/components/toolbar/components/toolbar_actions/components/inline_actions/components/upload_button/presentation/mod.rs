use crate::components::app::components::shell::components::toasts::{ToastOptions, use_toast};
use crate::services::customkeys::context::{use_custom_keys_service, use_upload_status};
use crate::services::customkeys::upload_status::UploadStatus;
use dioxus::prelude::*;

/// The file-import handler for the hidden upload input: reads the chosen file, imports it
/// through the sanctioned service command, and reports the outcome via toast. The visible
/// button (icon, label, click) comes from the shared toolbar-action set; this owns only the
/// import.
pub(super) fn use_upload_file_import() -> EventHandler<FormEvent> {
    let custom_keys_service = use_custom_keys_service();
    let mut upload_status = use_upload_status();
    let toast_api = use_toast();
    EventHandler::new(move |event: FormEvent| {
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
                    let outcome = custom_keys_service.import_overlay(contents.as_str());
                    let binding_count = outcome.binding_count();
                    let command_count = outcome.command_count();
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
    })
}
