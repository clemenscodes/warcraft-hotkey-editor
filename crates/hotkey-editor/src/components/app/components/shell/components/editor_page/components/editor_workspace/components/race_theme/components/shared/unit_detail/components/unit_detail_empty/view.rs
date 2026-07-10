/// The published `View` contract mirroring [`UnitDetailEmptyProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitDetailEmptyView {
    pub message: String,
}

impl ddd::View for UnitDetailEmptyView {}
