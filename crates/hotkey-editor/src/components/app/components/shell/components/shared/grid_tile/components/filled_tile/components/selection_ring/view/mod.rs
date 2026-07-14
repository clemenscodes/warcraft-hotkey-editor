#[derive(Clone, PartialEq)]
pub struct SelectionRingView {
    pub selected: bool,
}

impl ddd::View for SelectionRingView {}
