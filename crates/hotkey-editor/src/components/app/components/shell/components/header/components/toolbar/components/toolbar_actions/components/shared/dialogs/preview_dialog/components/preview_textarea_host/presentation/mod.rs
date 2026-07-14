use super::model::PreviewTextareaHostModel;
use crate::services::customkeys::context::use_custom_keys_service;
use dioxus::prelude::*;

pub(super) struct PreviewTextareaHostPresentation {
    pub(super) text: ReadSignal<String>,
}

pub(super) fn use_preview_textarea_host() -> PreviewTextareaHostPresentation {
    let custom_keys_service = use_custom_keys_service();
    let text = use_memo(move || custom_keys_service.exported_text());
    let text = text.into();
    PreviewTextareaHostPresentation { text }
}

impl ddd::Presentation for PreviewTextareaHostPresentation {
    type Model = PreviewTextareaHostModel;
}
