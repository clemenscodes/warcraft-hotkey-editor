use super::view::HelpLegendSectionView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_guide_host::data::HelpLegendEntry;
use dioxus::prelude::*;

/// The legend column's input: the toolbar rows to lay out.
#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendSectionModel {
    pub rows: &'static [HelpLegendEntry],
}

impl From<&HelpLegendSectionView> for HelpLegendSectionModel {
    fn from(view: &HelpLegendSectionView) -> Self {
        let HelpLegendSectionView { rows } = view.clone();
        Self { rows }
    }
}

impl ddd::Model for HelpLegendSectionModel {
    type View = HelpLegendSectionView;
}
