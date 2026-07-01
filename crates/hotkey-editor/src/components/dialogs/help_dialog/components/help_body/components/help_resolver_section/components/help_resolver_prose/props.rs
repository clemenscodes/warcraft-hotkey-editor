use dioxus::prelude::*;
use crate::components::dialogs::help_dialog::components::help_body::components::help_resolver_section::HelpResolverSectionProps;

/// The prose's input: the walkthrough paragraphs.
#[derive(Props, Clone, PartialEq)]
pub struct HelpResolverProseProps {
    pub paragraphs: &'static [&'static str],
}

impl From<&HelpResolverSectionProps> for HelpResolverProseProps {
    fn from(props: &HelpResolverSectionProps) -> Self {
        Self {
            paragraphs: props.prose,
        }
    }
}
