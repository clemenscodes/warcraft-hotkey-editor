use dioxus::prelude::*;

/// The body text's only input: the paragraph copy, passed as children.
#[derive(Props, Clone, PartialEq)]
pub struct HelpBodyTextProps {
    pub children: Element,
}
