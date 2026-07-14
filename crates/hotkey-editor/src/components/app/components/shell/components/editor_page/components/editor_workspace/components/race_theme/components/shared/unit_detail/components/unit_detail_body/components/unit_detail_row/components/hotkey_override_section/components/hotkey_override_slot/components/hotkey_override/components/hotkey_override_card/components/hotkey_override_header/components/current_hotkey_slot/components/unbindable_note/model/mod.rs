use super::view::UnbindableNoteView;
use dioxus::prelude::*;

/// The note shown for a passive ability that has no hotkey field.
#[derive(Props, Clone, PartialEq)]
pub struct UnbindableNoteModel {
    #[props(into)]
    pub text: String,
}

impl From<&UnbindableNoteView> for UnbindableNoteModel {
    fn from(view: &UnbindableNoteView) -> Self {
        let UnbindableNoteView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for UnbindableNoteModel {
    type View = UnbindableNoteView;
}
