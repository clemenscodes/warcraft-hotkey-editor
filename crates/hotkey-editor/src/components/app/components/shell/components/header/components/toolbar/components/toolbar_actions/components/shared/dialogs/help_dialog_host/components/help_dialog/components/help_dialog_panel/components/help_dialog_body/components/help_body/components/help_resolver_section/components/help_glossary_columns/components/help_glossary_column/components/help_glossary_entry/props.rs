use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::data::HelpGlossaryItem;
use dioxus::prelude::*;

/// One glossary entry's only input: the glossary item to render.
#[derive(Props, Clone, PartialEq)]
pub struct HelpGlossaryEntryProps {
    pub item: HelpGlossaryItem,
}
