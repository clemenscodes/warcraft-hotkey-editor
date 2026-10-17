#[derive(Clone, PartialEq)]
pub struct HelpLegendIconView {
    pub icon: &'static str,
}

impl ddd::Layered for HelpLegendIconView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HelpLegendIconView {}
