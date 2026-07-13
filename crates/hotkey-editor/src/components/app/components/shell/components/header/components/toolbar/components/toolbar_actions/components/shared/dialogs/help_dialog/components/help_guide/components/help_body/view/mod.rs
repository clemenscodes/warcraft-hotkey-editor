use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpContent;

/// The published `View` contract mirroring [`HelpBodyModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HelpBodyView {
    pub content: HelpContent,
}

impl ddd::View for HelpBodyView {}
