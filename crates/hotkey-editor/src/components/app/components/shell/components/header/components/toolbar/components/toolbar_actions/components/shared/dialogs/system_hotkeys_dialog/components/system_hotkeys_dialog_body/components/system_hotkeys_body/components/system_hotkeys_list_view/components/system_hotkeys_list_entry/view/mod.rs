use warcraft_keybinds::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct SystemHotkeysListEntryView {
    pub section_id: WarcraftObjectId,
    pub comment: String,
}

impl ddd::View for SystemHotkeysListEntryView {}
