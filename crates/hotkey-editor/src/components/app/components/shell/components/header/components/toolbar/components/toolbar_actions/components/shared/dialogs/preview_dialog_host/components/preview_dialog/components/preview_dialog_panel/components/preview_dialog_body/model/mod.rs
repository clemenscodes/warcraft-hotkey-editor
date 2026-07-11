use super::view::PreviewDialogBodyView;
use dioxus::prelude::*;

/// The preview dialog's scroll region input: the serialized text its textarea shows.
#[derive(Props, Clone, PartialEq)]
pub struct PreviewDialogBodyModel {
    pub text: ReadSignal<String>,
}

impl From<&PreviewDialogBodyView> for PreviewDialogBodyModel {
    fn from(view: &PreviewDialogBodyView) -> Self {
        let PreviewDialogBodyView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for PreviewDialogBodyModel {
    type View = PreviewDialogBodyView;
}
