use super::view::ConflictPanelView;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ConflictPanelModel {
    pub(crate) model: ConflictCardModel,
}

impl From<&ConflictPanelView> for ConflictPanelModel {
    fn from(view: &ConflictPanelView) -> Self {
        let ConflictPanelView { model } = view.clone();
        Self { model }
    }
}

impl ddd::Model for ConflictPanelModel {
    type View = ConflictPanelView;
}
