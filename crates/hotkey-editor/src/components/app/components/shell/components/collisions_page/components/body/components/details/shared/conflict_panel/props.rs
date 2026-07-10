use super::view::ConflictPanelView;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use dioxus::prelude::*;

/// The conflict card surface: the role caption over exactly one of the two clash
/// layouts (the pair row or the multi stack). Carries the shaped card model; the panel
/// splits it into the caption and the two layouts.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictPanelProps {
    pub(crate) model: ConflictCardModel,
}

impl From<&ConflictPanelView> for ConflictPanelProps {
    fn from(view: &ConflictPanelView) -> Self {
        let ConflictPanelView { model } = view.clone();
        Self { model }
    }
}

impl ddd::Props for ConflictPanelProps {
    type View = ConflictPanelView;
}
