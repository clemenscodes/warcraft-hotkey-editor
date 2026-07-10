use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_caption::ConflictCardCaptionProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_multi_stack::ConflictMultiStackProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_pair_row::ConflictPairRowProps;
use dioxus::prelude::*;

/// The conflict card surface: the role caption over exactly one of the two clash
/// layouts (the pair row or the multi stack).
#[derive(Props, Clone, PartialEq)]
pub struct ConflictPanelProps {
    pub caption: ConflictCardCaptionProps,
    pub pair_row: ConflictPairRowProps,
    pub multi_stack: ConflictMultiStackProps,
}

impl From<ConflictCardModel> for ConflictPanelProps {
    fn from(model: ConflictCardModel) -> Self {
        let caption = model.caption;
        let pair_row = model.pair_row;
        let multi_stack = model.multi_stack;
        Self {
            caption,
            pair_row,
            multi_stack,
        }
    }
}
