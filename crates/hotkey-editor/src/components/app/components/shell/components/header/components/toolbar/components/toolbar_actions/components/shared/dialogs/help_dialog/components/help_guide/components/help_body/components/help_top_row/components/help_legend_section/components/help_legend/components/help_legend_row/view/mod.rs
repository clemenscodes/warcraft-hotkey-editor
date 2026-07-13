use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpLegendEntry;

/// The published `View` contract mirroring [`HelpLegendRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HelpLegendRowView {
    pub entry: HelpLegendEntry,
}

impl ddd::View for HelpLegendRowView {}
