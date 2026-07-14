#[derive(Clone, PartialEq)]
pub struct ConflictUnitIconHostView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::View for ConflictUnitIconHostView {}
