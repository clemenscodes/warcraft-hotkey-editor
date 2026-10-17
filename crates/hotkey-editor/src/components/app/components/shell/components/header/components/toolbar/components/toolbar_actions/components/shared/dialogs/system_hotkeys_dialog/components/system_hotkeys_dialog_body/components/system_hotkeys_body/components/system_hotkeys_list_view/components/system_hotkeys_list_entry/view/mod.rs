use warcraft_keybinds::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct SystemHotkeysListEntryView {
    pub section_id: WarcraftObjectId,
    pub comment: String,
}

impl ddd::Layered for SystemHotkeysListEntryView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SystemHotkeysListEntryView {}
