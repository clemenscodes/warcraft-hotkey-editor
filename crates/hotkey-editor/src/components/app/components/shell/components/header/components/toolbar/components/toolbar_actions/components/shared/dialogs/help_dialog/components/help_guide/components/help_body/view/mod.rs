use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpContent;

#[derive(Clone, PartialEq)]
pub struct HelpBodyView {
    pub content: HelpContent,
}

impl ddd::Layered for HelpBodyView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HelpBodyView {}
