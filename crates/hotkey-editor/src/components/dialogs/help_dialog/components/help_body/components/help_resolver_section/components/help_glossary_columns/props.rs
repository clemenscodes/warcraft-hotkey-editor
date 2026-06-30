use dioxus::prelude::*;

use crate::components::dialogs::help_dialog::components::help_body::components::help_resolver_section::HelpResolverSectionProps;
use super::components::help_glossary_column::components::help_glossary_entry::HelpGlossaryEntryProps;

/// The glossary group's input: each column's entry list.
#[derive(Props, Clone, PartialEq)]
pub struct HelpGlossaryColumnsProps {
    pub columns: &'static [&'static [HelpGlossaryEntryProps]],
}

impl From<&HelpResolverSectionProps> for HelpGlossaryColumnsProps {
    fn from(props: &HelpResolverSectionProps) -> Self {
        Self {
            columns: props.glossary,
        }
    }
}
