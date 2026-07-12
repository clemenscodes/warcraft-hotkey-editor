use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_guide_host::data::HelpContent;

/// The published `View` contract mirroring [`HelpTopRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HelpTopRowView {
    pub content: HelpContent,
}

impl ddd::View for HelpTopRowView {}
