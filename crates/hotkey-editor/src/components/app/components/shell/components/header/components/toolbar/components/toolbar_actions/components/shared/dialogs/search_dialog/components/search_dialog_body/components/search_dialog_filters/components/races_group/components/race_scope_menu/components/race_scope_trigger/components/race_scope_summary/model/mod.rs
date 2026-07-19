use super::view::RaceScopeSummaryView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct RaceScopeSummaryModel {
    #[props(into)]
    pub summary: String,
}

impl From<&RaceScopeSummaryView> for RaceScopeSummaryModel {
    fn from(view: &RaceScopeSummaryView) -> Self {
        let RaceScopeSummaryView { summary } = view.clone();
        Self { summary }
    }
}

impl ddd::Model for RaceScopeSummaryModel {
    type View = RaceScopeSummaryView;
}
