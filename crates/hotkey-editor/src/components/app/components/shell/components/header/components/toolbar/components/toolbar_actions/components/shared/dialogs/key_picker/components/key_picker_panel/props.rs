use super::components::key_picker_body::KeyPickerBodyProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeaderProps;
use dioxus::prelude::*;

/// The key picker's bordered box: the header row above the scrolling board body,
/// wrapped in the library `DialogContent` (which carries no project class — this
/// panel's own classed `div` is the box).
#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerPanelProps {
    pub header: DialogHeaderProps,
    pub body: KeyPickerBodyProps,
}

impl From<&KeyPickerPanelProps> for DialogHeaderProps {
    fn from(props: &KeyPickerPanelProps) -> Self {
        props.header.clone()
    }
}

impl From<&KeyPickerPanelProps> for KeyPickerBodyProps {
    fn from(props: &KeyPickerPanelProps) -> Self {
        props.body.clone()
    }
}
