use super::view::PreviewDialogBodyView;
use dioxus::prelude::*;

/// The preview dialog's scroll region input: the serialized text its textarea shows.
#[derive(Props, Clone, PartialEq)]
pub struct PreviewDialogBodyProps {
    pub text: ReadSignal<String>,
}

impl From<&PreviewDialogBodyView> for PreviewDialogBodyProps {
    fn from(view: &PreviewDialogBodyView) -> Self {
        let PreviewDialogBodyView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for PreviewDialogBodyProps {
    type View = PreviewDialogBodyView;
}
