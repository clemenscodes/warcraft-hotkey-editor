use super::view::HelpLegendRowView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpLegendEntry;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendRowModel {
    pub entry: HelpLegendEntry,
}

impl From<&HelpLegendRowView> for HelpLegendRowModel {
    fn from(view: &HelpLegendRowView) -> Self {
        let HelpLegendRowView { entry } = view.clone();
        Self { entry }
    }
}

impl ddd::Model for HelpLegendRowModel {
    type View = HelpLegendRowView;
}
