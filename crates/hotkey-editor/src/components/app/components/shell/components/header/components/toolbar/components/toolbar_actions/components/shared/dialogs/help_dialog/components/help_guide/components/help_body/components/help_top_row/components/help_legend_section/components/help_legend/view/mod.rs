use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpLegendEntry;

#[derive(Clone, PartialEq)]
pub struct HelpLegendView {
    pub rows: &'static [HelpLegendEntry],
}

impl ddd::View for HelpLegendView {}
