#[derive(Clone, PartialEq)]
pub struct PagerCardNameView {
    pub name: String,
}

impl ddd::Layered for PagerCardNameView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PagerCardNameView {}
