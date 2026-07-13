use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpGlossaryItem;

/// The published `View` contract mirroring [`HelpGlossaryColumnModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HelpGlossaryColumnView {
    pub entries: &'static [HelpGlossaryItem],
}

impl ddd::View for HelpGlossaryColumnView {}
