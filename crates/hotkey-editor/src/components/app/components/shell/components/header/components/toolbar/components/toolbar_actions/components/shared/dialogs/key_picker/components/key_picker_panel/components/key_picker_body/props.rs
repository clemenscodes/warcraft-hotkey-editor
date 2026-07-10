use crate::components::app::components::shell::components::shared::key_picker_board::KeyPickerBoardProps;
use dioxus::prelude::*;

/// The key picker's scroll region input: the shared board it holds.
#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerBodyProps {
    pub board: KeyPickerBoardProps,
}

impl From<&KeyPickerBodyProps> for KeyPickerBoardProps {
    fn from(props: &KeyPickerBodyProps) -> Self {
        props.board.clone()
    }
}
