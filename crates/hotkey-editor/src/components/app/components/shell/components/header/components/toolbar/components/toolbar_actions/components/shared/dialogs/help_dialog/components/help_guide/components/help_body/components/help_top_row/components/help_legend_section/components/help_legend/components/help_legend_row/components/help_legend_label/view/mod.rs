#[derive(Clone, PartialEq)]
pub struct HelpLegendLabelView {
    pub label: String,
}

impl ddd::Layered for HelpLegendLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HelpLegendLabelView {}
