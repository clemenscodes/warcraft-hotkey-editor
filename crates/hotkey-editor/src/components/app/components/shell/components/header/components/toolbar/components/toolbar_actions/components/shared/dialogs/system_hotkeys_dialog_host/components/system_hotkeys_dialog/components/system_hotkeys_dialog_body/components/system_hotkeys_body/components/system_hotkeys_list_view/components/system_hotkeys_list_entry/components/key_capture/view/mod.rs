use warcraft_keybinds::WarcraftObjectId;

/// The published `View` contract mirroring [`KeyCaptureModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct KeyCaptureView {
    pub section_id: WarcraftObjectId,
}

impl ddd::View for KeyCaptureView {}
