use super::view::HelpBodyView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpContent;
use dioxus::prelude::*;

/// The body's input: the whole guide content, threaded down to the sections.
#[derive(Props, Clone, PartialEq)]
pub struct HelpBodyModel {
    pub content: HelpContent,
}

impl From<&HelpBodyView> for HelpBodyModel {
    fn from(view: &HelpBodyView) -> Self {
        let HelpBodyView { content } = view.clone();
        Self { content }
    }
}

impl ddd::Model for HelpBodyModel {
    type View = HelpBodyView;
}
