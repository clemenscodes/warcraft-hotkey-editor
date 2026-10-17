#[derive(Clone, PartialEq)]
pub struct RaceScopeSummaryView {
    pub summary: String,
}

impl ddd::Layered for RaceScopeSummaryView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for RaceScopeSummaryView {}
