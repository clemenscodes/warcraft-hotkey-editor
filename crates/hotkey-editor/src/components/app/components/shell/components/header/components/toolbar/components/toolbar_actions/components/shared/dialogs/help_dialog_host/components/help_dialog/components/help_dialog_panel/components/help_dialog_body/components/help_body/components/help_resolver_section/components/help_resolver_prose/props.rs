use dioxus::prelude::*;

/// The prose's input: the walkthrough paragraphs.
#[derive(Props, Clone, PartialEq)]
pub struct HelpResolverProseProps {
    pub paragraphs: &'static [&'static str],
}
