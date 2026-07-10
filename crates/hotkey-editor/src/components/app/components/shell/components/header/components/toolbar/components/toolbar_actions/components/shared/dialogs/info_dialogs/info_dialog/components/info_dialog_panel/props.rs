use super::components::info_dialog_body::InfoDialogBodyProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeaderProps;
use dioxus::prelude::*;

/// The info dialog's bordered box: the header row above the scrolling body, wrapped in
/// the library `DialogContent` (which carries no project class — this panel's own
/// classed `div` is the box).
#[derive(Props, Clone, PartialEq)]
pub struct InfoDialogPanelProps {
    pub header: DialogHeaderProps,
    pub body: InfoDialogBodyProps,
}

impl From<&InfoDialogPanelProps> for DialogHeaderProps {
    fn from(props: &InfoDialogPanelProps) -> Self {
        props.header.clone()
    }
}

impl From<&InfoDialogPanelProps> for InfoDialogBodyProps {
    fn from(props: &InfoDialogPanelProps) -> Self {
        props.body.clone()
    }
}
