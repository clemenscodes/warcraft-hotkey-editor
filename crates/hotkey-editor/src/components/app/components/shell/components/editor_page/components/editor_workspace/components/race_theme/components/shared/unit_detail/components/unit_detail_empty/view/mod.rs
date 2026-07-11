/// The published `View` contract mirroring [`UnitDetailEmptyModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitDetailEmptyView {
    pub message: String,
}

impl ddd::View for UnitDetailEmptyView {}
