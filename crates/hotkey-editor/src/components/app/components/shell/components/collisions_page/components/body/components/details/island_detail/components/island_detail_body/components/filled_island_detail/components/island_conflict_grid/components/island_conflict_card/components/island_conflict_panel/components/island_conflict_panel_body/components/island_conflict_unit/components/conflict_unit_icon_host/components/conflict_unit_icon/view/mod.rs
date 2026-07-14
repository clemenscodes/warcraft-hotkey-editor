#[derive(Clone, PartialEq)]
pub struct ConflictUnitIconView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::View for ConflictUnitIconView {}
