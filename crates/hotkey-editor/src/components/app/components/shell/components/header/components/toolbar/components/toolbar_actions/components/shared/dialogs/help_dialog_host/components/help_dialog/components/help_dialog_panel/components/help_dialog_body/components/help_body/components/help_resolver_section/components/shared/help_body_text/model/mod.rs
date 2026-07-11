use super::view::HelpBodyTextView;
use dioxus::prelude::*;

/// The body text's only input: the paragraph copy.
#[derive(Props, Clone, PartialEq)]
pub struct HelpBodyTextModel {
    #[props(into)]
    pub text: String,
}

impl From<&HelpBodyTextView> for HelpBodyTextModel {
    fn from(view: &HelpBodyTextView) -> Self {
        let HelpBodyTextView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for HelpBodyTextModel {
    type View = HelpBodyTextView;
}
