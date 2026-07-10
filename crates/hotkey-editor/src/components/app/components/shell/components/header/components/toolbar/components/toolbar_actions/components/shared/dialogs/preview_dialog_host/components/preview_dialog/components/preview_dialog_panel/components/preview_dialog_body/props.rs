use dioxus::prelude::*;

/// The preview dialog's scroll region input: the serialized text its textarea shows.
#[derive(Props, Clone, PartialEq)]
pub struct PreviewDialogBodyProps {
    pub text: ReadSignal<String>,
}
