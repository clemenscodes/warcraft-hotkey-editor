use warcraft_keybinds::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub struct KeyCaptureView {
    pub section_id: WarcraftObjectId,
}

impl ddd::Layered for KeyCaptureView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for KeyCaptureView {}
