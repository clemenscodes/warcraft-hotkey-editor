use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::data::HelpGlossaryItem;
use dioxus::prelude::*;

/// The glossary group's input: each column's item list.
#[derive(Props, Clone, PartialEq)]
pub struct HelpGlossaryColumnsProps {
    pub columns: &'static [&'static [HelpGlossaryItem]],
}
