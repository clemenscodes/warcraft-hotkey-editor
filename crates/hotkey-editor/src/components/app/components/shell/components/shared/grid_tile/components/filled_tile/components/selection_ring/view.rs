/// The published `View` contract mirroring [`SelectionRingProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SelectionRingView {
    pub selected: bool,
}

impl ddd::View for SelectionRingView {}
