use warcraft_keybinds::WarcraftObjectId;

/// The published `View` contract mirroring [`SystemHotkeysListEntryModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SystemHotkeysListEntryView {
    pub section_id: WarcraftObjectId,
    pub comment: String,
}

impl ddd::View for SystemHotkeysListEntryView {}
