use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpContent;

#[derive(Clone, PartialEq)]
pub struct HelpTopRowView {
    pub content: HelpContent,
}

impl ddd::View for HelpTopRowView {}
