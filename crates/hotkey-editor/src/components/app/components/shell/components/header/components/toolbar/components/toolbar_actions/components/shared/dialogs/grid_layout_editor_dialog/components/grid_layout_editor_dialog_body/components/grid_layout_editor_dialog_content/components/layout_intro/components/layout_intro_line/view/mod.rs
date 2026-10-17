#[derive(Clone, PartialEq)]
pub struct LayoutIntroLineView {
    pub line: String,
}

impl ddd::Layered for LayoutIntroLineView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for LayoutIntroLineView {}
