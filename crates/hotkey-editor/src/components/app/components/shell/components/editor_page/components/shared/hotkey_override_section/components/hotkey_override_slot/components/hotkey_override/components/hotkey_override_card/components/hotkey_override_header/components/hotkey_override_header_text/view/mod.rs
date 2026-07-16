use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct HotkeyOverrideHeaderTextView {
    pub name_text: String,
    pub object_id: WarcraftObjectId,
}

impl ddd::View for HotkeyOverrideHeaderTextView {}
