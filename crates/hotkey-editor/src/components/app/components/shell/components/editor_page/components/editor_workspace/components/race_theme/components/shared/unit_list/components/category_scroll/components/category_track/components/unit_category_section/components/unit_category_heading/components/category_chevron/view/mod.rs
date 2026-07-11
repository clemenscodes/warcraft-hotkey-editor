/// The published `View` contract mirroring [`CategoryChevronModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CategoryChevronView {
    pub is_collapsed: bool,
}

impl ddd::View for CategoryChevronView {}
