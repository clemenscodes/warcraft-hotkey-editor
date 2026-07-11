/// The published contract for the empty detail pane: the prompt it shows.
#[derive(Clone, PartialEq)]
pub struct EmptyUnitPositionDetailView {
    pub prompt: String,
}

impl ddd::View for EmptyUnitPositionDetailView {}
