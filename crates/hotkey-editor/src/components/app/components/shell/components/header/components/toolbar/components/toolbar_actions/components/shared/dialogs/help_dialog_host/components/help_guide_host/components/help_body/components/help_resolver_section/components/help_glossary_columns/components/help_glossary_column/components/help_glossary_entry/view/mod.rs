use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_guide_host::data::HelpGlossaryItem;

/// The published `View` contract mirroring [`HelpGlossaryEntryModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HelpGlossaryEntryView {
    pub item: HelpGlossaryItem,
}

impl ddd::View for HelpGlossaryEntryView {}
