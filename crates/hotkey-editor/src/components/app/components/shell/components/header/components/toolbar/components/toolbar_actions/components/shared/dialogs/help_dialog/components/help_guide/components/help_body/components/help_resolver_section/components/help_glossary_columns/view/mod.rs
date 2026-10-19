use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpGlossaryItem;

#[derive(Clone, PartialEq)]
pub struct HelpGlossaryColumnsView {
    pub columns: &'static [&'static [HelpGlossaryItem]],
}

impl ddd::Layered for HelpGlossaryColumnsView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HelpGlossaryColumnsView {}
