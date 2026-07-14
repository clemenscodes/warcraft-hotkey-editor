#[derive(Clone, PartialEq)]
pub struct DraggingSourceGhostView {
    pub active: bool,
}

impl ddd::View for DraggingSourceGhostView {}
