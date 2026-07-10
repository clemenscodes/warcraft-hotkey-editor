use super::view::EmptyMessageView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct EmptyMessageProps {
    #[props(into)]
    pub text: String,
}

impl From<&EmptyMessageView> for EmptyMessageProps {
    fn from(view: &EmptyMessageView) -> Self {
        let EmptyMessageView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for EmptyMessageProps {
    type View = EmptyMessageView;
}
