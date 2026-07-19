#[derive(Clone, PartialEq)]
pub struct RaceScopeSummaryView {
    pub summary: String,
}

impl ddd::View for RaceScopeSummaryView {}
