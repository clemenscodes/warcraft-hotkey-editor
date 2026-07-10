use dioxus::prelude::*;

/// The glossary term's only input: the term text.
#[derive(Props, Clone, PartialEq)]
pub struct HelpTermProps {
    #[props(into)]
    pub term: String,
}
