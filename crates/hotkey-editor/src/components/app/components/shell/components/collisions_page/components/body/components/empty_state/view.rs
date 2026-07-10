/// The published `View` contract mirroring [`EmptyStateProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct EmptyStateView {
    pub message: String,
}

impl ddd::View for EmptyStateView {}
