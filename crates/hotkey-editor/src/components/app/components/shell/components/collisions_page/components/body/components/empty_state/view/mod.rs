/// The published `View` contract mirroring [`EmptyStateModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct EmptyStateView {
    pub message: String,
}

impl ddd::View for EmptyStateView {}
