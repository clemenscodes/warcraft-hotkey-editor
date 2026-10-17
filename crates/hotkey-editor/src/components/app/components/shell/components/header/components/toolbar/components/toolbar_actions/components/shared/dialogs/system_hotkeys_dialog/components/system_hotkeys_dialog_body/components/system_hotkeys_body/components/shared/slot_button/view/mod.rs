use warcraft_keybinds::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct SlotButtonView {
    pub slot_label: String,
    pub section_id: WarcraftObjectId,
}

impl ddd::Layered for SlotButtonView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SlotButtonView {}
