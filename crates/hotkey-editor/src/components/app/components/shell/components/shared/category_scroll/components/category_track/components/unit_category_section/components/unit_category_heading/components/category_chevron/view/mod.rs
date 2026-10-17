#[derive(Clone, PartialEq)]
pub struct CategoryChevronView {
    pub is_collapsed: bool,
}

impl ddd::Layered for CategoryChevronView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CategoryChevronView {}
