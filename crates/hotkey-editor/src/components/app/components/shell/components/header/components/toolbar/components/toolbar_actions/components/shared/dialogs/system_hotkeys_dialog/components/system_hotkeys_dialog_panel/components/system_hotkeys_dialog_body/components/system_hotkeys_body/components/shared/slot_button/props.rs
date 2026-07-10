use dioxus::prelude::*;
use warcraft_keybinds::WarcraftObjectId;

/// A big WC3 slot used in the hero-selection and control-group layouts (edit on
/// click, no drag). Its binding + conflicts come from the CustomKeys query and its
/// editing section from the dialog state context; `compact` marks the tighter
/// control-group cell.
#[derive(Props, Clone, PartialEq)]
pub struct SlotButtonProps {
    pub slot_label: String,
    pub section_id: WarcraftObjectId,
    #[props(default = false)]
    pub compact: bool,
}
