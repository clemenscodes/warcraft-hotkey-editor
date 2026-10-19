#[derive(Clone, PartialEq)]
pub struct RaceTabLabelView {
    pub label: String,
}

impl ddd::Layered for RaceTabLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for RaceTabLabelView {}
