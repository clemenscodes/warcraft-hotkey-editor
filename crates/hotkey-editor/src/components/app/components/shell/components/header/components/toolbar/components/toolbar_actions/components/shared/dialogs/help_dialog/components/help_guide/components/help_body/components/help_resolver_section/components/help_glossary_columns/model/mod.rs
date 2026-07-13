use super::view::HelpGlossaryColumnsView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpGlossaryItem;
use dioxus::prelude::*;

/// The glossary group's input: each column's item list.
#[derive(Props, Clone, PartialEq)]
pub struct HelpGlossaryColumnsModel {
    pub columns: &'static [&'static [HelpGlossaryItem]],
}

impl From<&HelpGlossaryColumnsView> for HelpGlossaryColumnsModel {
    fn from(view: &HelpGlossaryColumnsView) -> Self {
        let HelpGlossaryColumnsView { columns } = view.clone();
        Self { columns }
    }
}

impl ddd::Model for HelpGlossaryColumnsModel {
    type View = HelpGlossaryColumnsView;
}
