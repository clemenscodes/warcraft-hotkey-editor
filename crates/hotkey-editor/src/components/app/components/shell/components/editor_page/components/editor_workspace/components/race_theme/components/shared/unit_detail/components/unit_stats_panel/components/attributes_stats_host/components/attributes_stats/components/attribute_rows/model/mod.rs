use super::view::AttributeRowsView;
use dioxus::prelude::*;
use warcraft_api::AttributeStatistic;

#[derive(Props, Clone, PartialEq)]
pub struct AttributeRowsModel {
    pub strength: AttributeStatistic,
    pub strength_is_primary: bool,
    pub agility: AttributeStatistic,
    pub agility_is_primary: bool,
    pub intelligence: AttributeStatistic,
    pub intelligence_is_primary: bool,
}

impl From<&AttributeRowsView> for AttributeRowsModel {
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

impl ddd::Model for AttributeRowsModel {
    type View = AttributeRowsView;
}
