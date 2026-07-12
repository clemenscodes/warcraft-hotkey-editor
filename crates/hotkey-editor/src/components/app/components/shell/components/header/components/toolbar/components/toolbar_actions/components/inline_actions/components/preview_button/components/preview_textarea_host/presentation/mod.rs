use super::model::PreviewTextareaHostModel;
use crate::services::customkeys::context::use_loaded_keys;
use dioxus::prelude::*;

/// The preview host's shaped data: the serialized document text the textarea shows.
pub(super) struct PreviewTextareaHostPresentation {
    pub(super) text: ReadSignal<String>,
}

/// Serializes the loaded document as-is behind a single flat call — the one piece of work
/// the body is not allowed to do. It serializes the stored document without
/// re-normalizing (R5): the loaded-keys signal mirrors localStorage (R1) and that text is
/// already normalized (R2), so this is exactly the stored text.
pub(super) fn use_preview_textarea_host() -> PreviewTextareaHostPresentation {
    let loaded_keys = use_loaded_keys();
    let text = use_memo(move || {
        let loaded = loaded_keys.read();
        match loaded.as_ref() {
            Some(file) => file.to_string(),
            None => String::new(),
        }
    });
    let text = text.into();
    PreviewTextareaHostPresentation { text }
}

impl ddd::Presentation for PreviewTextareaHostPresentation {
    type Model = PreviewTextareaHostModel;
}
