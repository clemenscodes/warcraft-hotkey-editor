use super::view::{IslandConflictCardData, IslandConflictPanelBodyView};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictPanelBodyModel {
    pub(crate) cards: Vec<IslandConflictCardData>,
}

impl From<&IslandConflictPanelBodyView> for IslandConflictPanelBodyModel {
    fn from(view: &IslandConflictPanelBodyView) -> Self {
        let IslandConflictPanelBodyView { cards } = view.clone();
        Self { cards }
    }
}

impl ddd::Layered for IslandConflictPanelBodyModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for IslandConflictPanelBodyModel {
    type View = IslandConflictPanelBodyView;
}
