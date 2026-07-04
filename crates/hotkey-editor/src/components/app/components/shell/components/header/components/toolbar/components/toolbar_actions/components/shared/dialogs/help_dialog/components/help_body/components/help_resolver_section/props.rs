use dioxus::prelude::*;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_body::HelpBodyProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_body::components::help_resolver_section::components::help_glossary_columns::components::help_glossary_column::components::help_glossary_entry::HelpGlossaryEntryProps;

/// The resolver section's input: the walkthrough paragraphs and the glossary
/// columns.
#[derive(Props, Clone, PartialEq)]
pub struct HelpResolverSectionProps {
    pub prose: &'static [&'static str],
    pub glossary: &'static [&'static [HelpGlossaryEntryProps]],
}

impl From<&HelpBodyProps> for HelpResolverSectionProps {
    fn from(props: &HelpBodyProps) -> Self {
        Self {
            prose: props.content.resolver_prose,
            glossary: props.content.glossary,
        }
    }
}
