use super::data::ARIA_LABEL;
use crate::components::app::components::shell::components::shared::icons::ICON_UPLOAD;
use crate::components::app::components::shell::components::toasts::{ToastOptions, use_toast};
use crate::services::customkeys::context::{use_custom_keys_service, use_upload_status};
use crate::services::customkeys::upload_status::UploadStatus;
use dioxus::prelude::*;

/// The file-import handler for the hidden upload input: reads the chosen file, imports it
/// through the sanctioned service command, and reports the outcome via toast.
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

/// The upload button's shaped data: the fixed icon and label, whether the import-info dialog is
/// open, the click handler that opens it, the change handler the mounted dialog mirrors its own
/// close through, and the hidden input's file-import handler. The open signal is local and owned
/// here — the button that opens the dialog owns it, so the dialog travels with it.
pub(super) struct UploadButtonPresentation {
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) open: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) on_open_change: Callback<bool>,
    pub(super) on_change: EventHandler<FormEvent>,
}

/// Owns the import-info dialog's local open signal and shapes the button's data: the click
/// handler that opens the dialog, the change handler the mounted dialog mirrors its own close
/// through, and the hidden input's file-import handler.
pub(super) fn use_upload_button() -> UploadButtonPresentation {
    let on_change = use_upload_file_import();
    let mut open_signal = use_signal::<bool>(|| false);
    let open = open_signal();
    let onclick = EventHandler::new(move |_event: MouseEvent| open_signal.set(true));
    let on_open_change = Callback::new(move |is_open: bool| open_signal.set(is_open));
    UploadButtonPresentation {
        icon: ICON_UPLOAD,
        aria_label: ARIA_LABEL,
        open,
        onclick,
        on_open_change,
        on_change,
    }
}
