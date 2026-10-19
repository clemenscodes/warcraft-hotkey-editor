#[derive(Clone, PartialEq)]
pub struct FilterGroupLabelView {
    pub label: &'static str,
}

impl ddd::Layered for FilterGroupLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for FilterGroupLabelView {}
