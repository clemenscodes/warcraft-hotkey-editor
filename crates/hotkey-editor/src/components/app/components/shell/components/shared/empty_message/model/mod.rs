use super::view::EmptyMessageView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct EmptyMessageModel {
    #[props(into)]
    pub text: String,
}

impl From<&EmptyMessageView> for EmptyMessageModel {
    fn from(view: &EmptyMessageView) -> Self {
        let EmptyMessageView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for EmptyMessageModel {
    type View = EmptyMessageView;
}
