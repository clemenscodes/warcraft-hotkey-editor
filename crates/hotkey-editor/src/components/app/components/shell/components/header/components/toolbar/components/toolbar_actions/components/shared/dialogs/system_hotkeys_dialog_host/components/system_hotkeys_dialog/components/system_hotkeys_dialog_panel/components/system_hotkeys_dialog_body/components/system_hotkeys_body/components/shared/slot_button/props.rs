use super::view::SlotButtonView;
use dioxus::prelude::*;
use warcraft_keybinds::WarcraftObjectId;

/// A big WC3 slot used in the hero-selection and control-group layouts (edit on
/// click, no drag). Its binding + conflicts come from the CustomKeys query and its
/// editing section from the dialog state context. The cell's size (including the
/// tighter control-group density) is owned by the parent row, so no density flag
/// rides here.
#[derive(Props, Clone, PartialEq)]
pub struct SlotButtonProps {
    pub slot_label: String,
    pub section_id: WarcraftObjectId,
}

impl From<&SlotButtonView> for SlotButtonProps {
    fn from(view: &SlotButtonView) -> Self {
        let SlotButtonView {
            slot_label,
            section_id,
        } = view.clone();
        Self {
            slot_label,
            section_id,
        }
    }
}

impl ddd::Props for SlotButtonProps {
    type View = SlotButtonView;
}
