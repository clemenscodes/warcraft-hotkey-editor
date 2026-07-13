use super::model::PreviewTextareaHostModel;
use crate::services::customkeys::context::use_custom_keys_service;
use dioxus::prelude::*;

/// The preview host's shaped data: the stored document text the textarea shows.
pub(super) struct PreviewTextareaHostPresentation {
    pub(super) text: ReadSignal<String>,
}

/// Returns the exact stored `CustomKeys.txt` text behind a single flat call — the one
/// piece of work the body is not allowed to do. It reads through the service's
/// `exported_text` (R5: preview IS the stored localStorage text, nothing more — no
/// re-serialize, no re-normalize). Reading subscribes to the aggregate, so the textarea
/// re-reads on every mutation.
pub(super) fn use_preview_textarea_host() -> PreviewTextareaHostPresentation {
    let custom_keys_service = use_custom_keys_service();
    let text = use_memo(move || custom_keys_service.exported_text());
    let text = text.into();
    PreviewTextareaHostPresentation { text }
}

impl ddd::Presentation for PreviewTextareaHostPresentation {
    type Model = PreviewTextareaHostModel;
}
