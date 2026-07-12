use super::view::ConflictPanelBodyView;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use dioxus::prelude::*;

/// The conflict panel card's body region input: the shaped conflict card model carried as a
/// list so the region is `Default`-able. Exactly one model is present in practice; the body
/// renders its caption over the pair-vs-multi clash layout.
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
