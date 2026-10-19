#[derive(Clone, PartialEq)]
pub struct EmptyDetailView {
    pub prompt: String,
}

impl ddd::Layered for EmptyDetailView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for EmptyDetailView {}
