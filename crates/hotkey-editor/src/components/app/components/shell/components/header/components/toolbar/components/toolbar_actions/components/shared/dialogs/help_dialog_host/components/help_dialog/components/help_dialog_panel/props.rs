use super::components::help_dialog_body::HelpDialogBodyProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeaderProps;
use dioxus::prelude::*;

/// The help dialog's bordered box: the header row above the scrolling body, wrapped in
/// the library `DialogContent` (which carries no project class — this panel's own
/// classed `div` is the box).
#[derive(Props, Clone, PartialEq)]
pub struct HelpDialogPanelProps {
    pub header: DialogHeaderProps,
    pub body: HelpDialogBodyProps,
}

impl From<&HelpDialogPanelProps> for DialogHeaderProps {
    fn from(props: &HelpDialogPanelProps) -> Self {
        props.header.clone()
    }
}

impl From<&HelpDialogPanelProps> for HelpDialogBodyProps {
    fn from(props: &HelpDialogPanelProps) -> Self {
        props.body.clone()
    }
}
