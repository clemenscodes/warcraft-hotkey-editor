/// The published `View` contract mirroring [`ConflictUnitIconModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictUnitIconView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::View for ConflictUnitIconView {}
