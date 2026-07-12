use super::view::HelpLegendView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_guide_host::data::HelpLegendEntry;
use dioxus::prelude::*;

/// The legend's input: the toolbar rows to render.
#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendModel {
    pub rows: &'static [HelpLegendEntry],
}

impl From<&HelpLegendView> for HelpLegendModel {
    fn from(view: &HelpLegendView) -> Self {
        let HelpLegendView { rows } = view.clone();
        Self { rows }
    }
}

impl ddd::Model for HelpLegendModel {
    type View = HelpLegendView;
}
