#[derive(Clone, PartialEq)]
pub struct ResolveSectionTabLabelView {
    pub text: String,
}

impl ddd::Layered for ResolveSectionTabLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ResolveSectionTabLabelView {}
