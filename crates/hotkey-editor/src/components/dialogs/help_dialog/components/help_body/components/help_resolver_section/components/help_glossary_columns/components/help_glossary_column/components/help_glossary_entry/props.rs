use dioxus::prelude::*;

/// One glossary entry's inputs: the term and its definition.
#[derive(Props, Clone, PartialEq)]
pub struct HelpGlossaryEntryProps {
    pub term: &'static str,
    pub description: &'static str,
}
