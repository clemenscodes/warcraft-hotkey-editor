#[derive(Clone, PartialEq)]
pub struct EmptyStateView {
    pub message: String,
}

impl ddd::Layered for EmptyStateView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for EmptyStateView {}
