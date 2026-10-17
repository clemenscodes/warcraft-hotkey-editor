#[derive(Clone, PartialEq)]
pub struct HelpLegendDescriptionView {
    pub description: String,
}

impl ddd::Layered for HelpLegendDescriptionView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HelpLegendDescriptionView {}
