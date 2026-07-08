use dioxus::prelude::*;

/// The body text's only input: the paragraph copy.
#[derive(Props, Clone, PartialEq)]
pub struct HelpBodyTextProps {
    #[props(into)]
    pub text: String,
}
