/// The published `View` contract mirroring [`ConflictDetailUnitIconModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictDetailUnitIconView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::View for ConflictDetailUnitIconView {}
