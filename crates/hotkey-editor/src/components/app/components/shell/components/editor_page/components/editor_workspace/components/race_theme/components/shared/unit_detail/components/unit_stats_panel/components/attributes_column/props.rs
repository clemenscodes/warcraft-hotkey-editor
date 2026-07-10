use super::view::AttributesColumnView;
use dioxus::prelude::*;
use warcraft_keybinds::HeroStatistics;

/// The hero attributes column's input: the hero's three attributes at the selected
/// level, or `None` for a non-hero unit (the column then renders nothing).
#[derive(Props, Clone, PartialEq)]
pub struct AttributesColumnProps {
    pub hero: Option<HeroStatistics>,
}

impl From<&AttributesColumnView> for AttributesColumnProps {
    fn from(view: &AttributesColumnView) -> Self {
        let AttributesColumnView { hero } = view.clone();
        Self { hero }
    }
}

impl ddd::Props for AttributesColumnProps {
    type View = AttributesColumnView;
}
