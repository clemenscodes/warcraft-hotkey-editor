use super::components::preview_dialog_body::PreviewDialogBodyProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeaderProps;
use dioxus::prelude::*;

/// The preview dialog's bordered box: the header row above the scrolling body, wrapped
/// in the library `DialogContent` (which carries no project class — this panel's own
/// classed `div` is the box).
#[derive(Props, Clone, PartialEq)]
pub struct PreviewDialogPanelProps {
    pub header: DialogHeaderProps,
    pub body: PreviewDialogBodyProps,
}

impl From<&PreviewDialogPanelProps> for DialogHeaderProps {
    fn from(props: &PreviewDialogPanelProps) -> Self {
        props.header.clone()
    }
}

impl From<&PreviewDialogPanelProps> for PreviewDialogBodyProps {
    fn from(props: &PreviewDialogPanelProps) -> Self {
        props.body.clone()
    }
}
