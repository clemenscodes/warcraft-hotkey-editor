use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpGlossaryItem;

#[derive(Clone, PartialEq)]
pub struct HelpGlossaryColumnView {
    pub entries: &'static [HelpGlossaryItem],
}

impl ddd::View for HelpGlossaryColumnView {}
