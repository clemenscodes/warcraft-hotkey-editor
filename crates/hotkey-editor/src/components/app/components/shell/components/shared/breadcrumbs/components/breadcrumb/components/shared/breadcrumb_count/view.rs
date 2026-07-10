/// The published `View` contract mirroring [`BreadcrumbCountProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BreadcrumbCountView {
    pub count: usize,
}

impl ddd::View for BreadcrumbCountView {}
