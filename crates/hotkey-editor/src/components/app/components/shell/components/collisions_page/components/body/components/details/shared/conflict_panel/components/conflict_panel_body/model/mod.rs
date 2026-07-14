use super::view::ConflictPanelBodyView;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ConflictPanelBodyModel {
    pub(crate) models: Vec<ConflictCardModel>,
}

impl From<&ConflictPanelBodyView> for ConflictPanelBodyModel {
    fn from(view: &ConflictPanelBodyView) -> Self {
        let ConflictPanelBodyView { models } = view.clone();
        Self { models }
    }
}

impl ddd::Model for ConflictPanelBodyModel {
    type View = ConflictPanelBodyView;
}
