#[derive(Clone, PartialEq)]
pub struct ConflictDetailUnitIconView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::View for ConflictDetailUnitIconView {}
