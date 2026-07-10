use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::data::HelpGlossaryItem;
use dioxus::prelude::*;

/// The resolver section's input: the walkthrough paragraphs and the glossary
/// columns.
#[derive(Props, Clone, PartialEq)]
pub struct HelpResolverSectionProps {
    pub prose: &'static [&'static str],
    pub glossary: &'static [&'static [HelpGlossaryItem]],
}
