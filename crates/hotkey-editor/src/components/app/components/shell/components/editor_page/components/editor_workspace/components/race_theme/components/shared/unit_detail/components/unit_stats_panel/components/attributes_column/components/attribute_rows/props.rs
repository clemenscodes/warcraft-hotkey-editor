use super::view::AttributeRowsView;
use dioxus::prelude::*;
use warcraft_keybinds::AttributeStatistic;

/// The attribute rows' input: each of the hero's three attributes at the selected level
/// and whether it is the hero's primary attribute (which drives the row's gold glow).
/// The runtime primary comparison is resolved here rather than in a component body.
#[derive(Props, Clone, PartialEq)]
pub struct AttributeRowsProps {
    pub strength: AttributeStatistic,
    pub strength_is_primary: bool,
    pub agility: AttributeStatistic,
    pub agility_is_primary: bool,
    pub intelligence: AttributeStatistic,
    pub intelligence_is_primary: bool,
}

impl From<&AttributeRowsView> for AttributeRowsProps {
    fn from(view: &AttributeRowsView) -> Self {
        let AttributeRowsView {
            strength,
            strength_is_primary,
            agility,
            agility_is_primary,
            intelligence,
            intelligence_is_primary,
        } = view.clone();
        Self {
            strength,
            strength_is_primary,
            agility,
            agility_is_primary,
            intelligence,
            intelligence_is_primary,
        }
    }
}

impl ddd::Props for AttributeRowsProps {
    type View = AttributeRowsView;
}
