use super::view::HelpLegendSectionView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpLegendEntry;
use dioxus::prelude::*;

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
