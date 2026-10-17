#[derive(Clone, PartialEq)]
pub struct ResolveSectionTabCountView {
    pub count: usize,
}

impl ddd::Layered for ResolveSectionTabCountView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ResolveSectionTabCountView {}
