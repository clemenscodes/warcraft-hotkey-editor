#[derive(Clone, PartialEq)]
pub struct SystemHotkeysListEntryLabelView {
    pub text: String,
}

impl ddd::Layered for SystemHotkeysListEntryLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SystemHotkeysListEntryLabelView {}
