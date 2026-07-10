use crate::components::app::components::shell::components::shared::key_picker_board::KeyPickerBoardProps;
use dioxus::prelude::*;

/// The system key picker's scroll region input: the board props (both keyboard
/// columns plus the pick and dismiss handlers) it hands to the shared board host.
#[derive(Props, Clone, PartialEq)]
pub struct SystemKeyPickerDialogBodyProps {
    pub board: KeyPickerBoardProps,
}

impl From<&SystemKeyPickerDialogBodyProps> for KeyPickerBoardProps {
    fn from(props: &SystemKeyPickerDialogBodyProps) -> Self {
        props.board.clone()
    }
}
