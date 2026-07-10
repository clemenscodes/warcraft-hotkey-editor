use super::view::HelpBodyTextView;
use dioxus::prelude::*;

/// The body text's only input: the paragraph copy.
#[derive(Props, Clone, PartialEq)]
pub struct HelpBodyTextProps {
    #[props(into)]
    pub text: String,
}

impl From<&HelpBodyTextView> for HelpBodyTextProps {
    fn from(view: &HelpBodyTextView) -> Self {
        let HelpBodyTextView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for HelpBodyTextProps {
    type View = HelpBodyTextView;
}
