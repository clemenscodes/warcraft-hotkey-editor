#[derive(Clone, PartialEq)]
pub struct SelectionRingView {
    pub selected: bool,
}

impl ddd::Layered for SelectionRingView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SelectionRingView {}
