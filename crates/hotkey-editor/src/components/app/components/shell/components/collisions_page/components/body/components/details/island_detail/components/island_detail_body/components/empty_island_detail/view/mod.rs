#[derive(Clone, PartialEq)]
pub struct EmptyIslandDetailView {
    pub prompt: String,
}

impl ddd::Layered for EmptyIslandDetailView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for EmptyIslandDetailView {}
