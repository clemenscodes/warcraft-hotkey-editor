use super::view::HelpGlossaryEntryView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_guide_host::data::HelpGlossaryItem;
use dioxus::prelude::*;

/// One glossary entry's only input: the glossary item to render.
#[derive(Props, Clone, PartialEq)]
pub struct HelpGlossaryEntryModel {
    pub item: HelpGlossaryItem,
}

impl From<&HelpGlossaryEntryView> for HelpGlossaryEntryModel {
    fn from(view: &HelpGlossaryEntryView) -> Self {
        let HelpGlossaryEntryView { item } = view.clone();
        Self { item }
    }
}

impl ddd::Model for HelpGlossaryEntryModel {
    type View = HelpGlossaryEntryView;
}
