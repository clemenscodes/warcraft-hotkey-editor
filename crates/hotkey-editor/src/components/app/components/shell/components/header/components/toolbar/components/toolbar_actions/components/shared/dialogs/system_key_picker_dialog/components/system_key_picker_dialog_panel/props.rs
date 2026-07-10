use super::components::system_key_picker_dialog_body::SystemKeyPickerDialogBodyProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeaderProps;
use dioxus::prelude::*;

/// The system key picker's bordered box: the header row above the scrolling board body,
/// wrapped in the library `DialogContent` (which carries no project class — this panel's
/// own classed `div` is the box).
#[derive(Props, Clone, PartialEq)]
pub struct SystemKeyPickerDialogPanelProps {
    pub header: DialogHeaderProps,
    pub body: SystemKeyPickerDialogBodyProps,
}

impl From<&SystemKeyPickerDialogPanelProps> for DialogHeaderProps {
    fn from(props: &SystemKeyPickerDialogPanelProps) -> Self {
        props.header.clone()
    }
}

impl From<&SystemKeyPickerDialogPanelProps> for SystemKeyPickerDialogBodyProps {
    fn from(props: &SystemKeyPickerDialogPanelProps) -> Self {
        props.body.clone()
    }
}
