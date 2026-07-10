use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;

/// The published `View` contract mirroring [`ConflictPanelProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictPanelView {
    pub(crate) model: ConflictCardModel,
}

impl ddd::View for ConflictPanelView {}
