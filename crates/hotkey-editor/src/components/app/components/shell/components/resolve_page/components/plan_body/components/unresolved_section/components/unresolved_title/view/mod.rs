/// The published `View` contract mirroring [`UnresolvedTitleModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnresolvedTitleView {
    pub text: &'static str,
}

impl ddd::View for UnresolvedTitleView {}
