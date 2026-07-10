use super::view::HelpGlossaryColumnView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::data::HelpGlossaryItem;
use dioxus::prelude::*;

/// One column's only input: the glossary items it lays out, in order.
#[derive(Props, Clone, PartialEq)]
pub struct HelpGlossaryColumnProps {
    pub entries: &'static [HelpGlossaryItem],
}

impl From<&HelpGlossaryColumnView> for HelpGlossaryColumnProps {
    fn from(view: &HelpGlossaryColumnView) -> Self {
        let HelpGlossaryColumnView { entries } = view.clone();
        Self { entries }
    }
}

impl ddd::Props for HelpGlossaryColumnProps {
    type View = HelpGlossaryColumnView;
}
