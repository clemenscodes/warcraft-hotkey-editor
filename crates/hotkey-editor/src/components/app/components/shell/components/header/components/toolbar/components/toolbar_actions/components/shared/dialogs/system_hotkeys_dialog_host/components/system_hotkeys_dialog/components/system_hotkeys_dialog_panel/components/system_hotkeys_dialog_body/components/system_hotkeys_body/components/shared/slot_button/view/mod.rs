use warcraft_keybinds::WarcraftObjectId;

/// The published `View` contract mirroring [`SlotButtonModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SlotButtonView {
    pub slot_label: String,
    pub section_id: WarcraftObjectId,
}

impl ddd::View for SlotButtonView {}
