use super::components::key_picker_key::KeyPickerKeyProps;
use dioxus::prelude::*;

/// One row of the picker board: its already-shaped keys, built by the picker hook.
#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerRowProps {
    pub keys: Vec<KeyPickerKeyProps>,
}
