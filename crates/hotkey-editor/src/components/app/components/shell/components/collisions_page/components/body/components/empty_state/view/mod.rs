#[derive(Clone, PartialEq)]
pub struct EmptyStateView {
    pub message: String,
}

impl ddd::View for EmptyStateView {}
