use dioxus::prelude::*;

/// The glossary term's only input: the term text, passed as children.
#[derive(Props, Clone, PartialEq)]
pub struct HelpTermProps {
    pub children: Element,
}
