use super::components::system_key_picker_key::SystemKeyPickerKeyProps;
use dioxus::prelude::*;

/// One row of the system keyboard board: its already-shaped keys, built by the
/// picker hook.
#[derive(Props, Clone, PartialEq)]
pub struct SystemKeyPickerRowProps {
    pub keys: Vec<SystemKeyPickerKeyProps>,
}
