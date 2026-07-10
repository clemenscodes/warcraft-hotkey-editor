use super::view::HelpResolverProseView;
use dioxus::prelude::*;

/// The prose's input: the walkthrough paragraphs.
#[derive(Props, Clone, PartialEq)]
pub struct HelpResolverProseProps {
    pub paragraphs: &'static [&'static str],
}

impl From<&HelpResolverProseView> for HelpResolverProseProps {
    fn from(view: &HelpResolverProseView) -> Self {
        let HelpResolverProseView { paragraphs } = view.clone();
        Self { paragraphs }
    }
}

impl ddd::Props for HelpResolverProseProps {
    type View = HelpResolverProseView;
}
