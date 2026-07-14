use super::view::AttributesStatsView;
use dioxus::prelude::*;
use warcraft_api::HeroStatistics;

#[derive(Props, Clone, PartialEq)]
pub struct AttributesStatsModel {
    pub hero: Option<HeroStatistics>,
}

impl From<&AttributesStatsView> for AttributesStatsModel {
    fn from(view: &AttributesStatsView) -> Self {
        let AttributesStatsView { hero } = view.clone();
        Self { hero }
    }
}

impl ddd::Model for AttributesStatsModel {
    type View = AttributesStatsView;
}
