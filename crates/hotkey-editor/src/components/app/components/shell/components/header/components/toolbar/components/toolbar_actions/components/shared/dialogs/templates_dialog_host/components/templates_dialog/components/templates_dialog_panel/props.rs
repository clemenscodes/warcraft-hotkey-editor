use super::components::templates_dialog_body::TemplatesDialogBodyProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeaderProps;
use dioxus::prelude::*;

/// The templates dialog's bordered box: the header row above the scrolling body, wrapped
/// in the library `DialogContent` (which carries no project class — this panel's own
/// classed `div` is the box).
#[derive(Props, Clone, PartialEq)]
pub struct TemplatesDialogPanelProps {
    pub header: DialogHeaderProps,
    pub body: TemplatesDialogBodyProps,
}

impl From<&TemplatesDialogPanelProps> for DialogHeaderProps {
    fn from(props: &TemplatesDialogPanelProps) -> Self {
        props.header.clone()
    }
}

impl From<&TemplatesDialogPanelProps> for TemplatesDialogBodyProps {
    fn from(props: &TemplatesDialogPanelProps) -> Self {
        props.body.clone()
    }
}
