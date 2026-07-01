use crate::components::dialogs::help_dialog::components::help_body::HelpBodyProps;
use crate::components::dialogs::help_dialog::data::HelpContent;
use dioxus::prelude::*;

/// The top row's input: the content, split between its two columns.
#[derive(Props, Clone, PartialEq)]
pub struct HelpTopRowProps {
    pub content: HelpContent,
}

impl From<&HelpBodyProps> for HelpTopRowProps {
    fn from(props: &HelpBodyProps) -> Self {
        Self {
            content: props.content,
        }
    }
}
