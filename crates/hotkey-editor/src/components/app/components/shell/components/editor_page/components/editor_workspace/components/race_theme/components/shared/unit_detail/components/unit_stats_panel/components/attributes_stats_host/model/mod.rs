use super::view::AttributesStatsHostView;
use dioxus::prelude::*;
use warcraft_api::HeroStatistics;

#[derive(Props, Clone, PartialEq)]
pub struct AttributesStatsHostModel {
    pub hero: Option<HeroStatistics>,
}

impl From<&AttributesStatsHostView> for AttributesStatsHostModel {
    fn from(view: &AttributesStatsHostView) -> Self {
        let AttributesStatsHostView { hero } = view.clone();
        Self { hero }
    }
}

impl ddd::Model for AttributesStatsHostModel {
    type View = AttributesStatsHostView;
}
