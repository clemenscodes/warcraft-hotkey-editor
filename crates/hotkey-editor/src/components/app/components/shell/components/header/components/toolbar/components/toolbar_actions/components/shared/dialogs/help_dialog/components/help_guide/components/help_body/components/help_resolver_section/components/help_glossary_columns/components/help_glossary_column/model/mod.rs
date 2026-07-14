use super::view::HelpGlossaryColumnView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpGlossaryItem;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HelpGlossaryColumnModel {
    pub entries: &'static [HelpGlossaryItem],
}

impl From<&HelpGlossaryColumnView> for HelpGlossaryColumnModel {
    fn from(view: &HelpGlossaryColumnView) -> Self {
        let HelpGlossaryColumnView { entries } = view.clone();
        Self { entries }
    }
}

impl ddd::Model for HelpGlossaryColumnModel {
    type View = HelpGlossaryColumnView;
}
