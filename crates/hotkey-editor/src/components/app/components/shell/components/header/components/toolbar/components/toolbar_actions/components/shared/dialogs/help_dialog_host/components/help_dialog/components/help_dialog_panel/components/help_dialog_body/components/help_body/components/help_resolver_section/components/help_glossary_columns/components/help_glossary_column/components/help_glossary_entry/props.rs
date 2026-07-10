use super::view::HelpGlossaryEntryView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::data::HelpGlossaryItem;
use dioxus::prelude::*;

/// One glossary entry's only input: the glossary item to render.
#[derive(Props, Clone, PartialEq)]
pub struct HelpGlossaryEntryProps {
    pub item: HelpGlossaryItem,
}

impl From<&HelpGlossaryEntryView> for HelpGlossaryEntryProps {
    fn from(view: &HelpGlossaryEntryView) -> Self {
        let HelpGlossaryEntryView { item } = view.clone();
        Self { item }
    }
}

impl ddd::Props for HelpGlossaryEntryProps {
    type View = HelpGlossaryEntryView;
}
