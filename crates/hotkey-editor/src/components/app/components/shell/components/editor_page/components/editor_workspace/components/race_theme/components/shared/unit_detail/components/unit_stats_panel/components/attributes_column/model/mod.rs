use super::view::AttributesColumnView;
use dioxus::prelude::*;
use warcraft_api::HeroStatistics;

/// The hero attributes column's input: the hero's three attributes at the selected
/// level, or `None` for a non-hero unit (the column then renders nothing).
#[derive(Props, Clone, PartialEq)]
pub struct AttributesColumnModel {
    pub hero: Option<HeroStatistics>,
}

impl From<&AttributesColumnView> for AttributesColumnModel {
    fn from(view: &AttributesColumnView) -> Self {
        let AttributesColumnView { hero } = view.clone();
        Self { hero }
    }
}

impl ddd::Model for AttributesColumnModel {
    type View = AttributesColumnView;
}
