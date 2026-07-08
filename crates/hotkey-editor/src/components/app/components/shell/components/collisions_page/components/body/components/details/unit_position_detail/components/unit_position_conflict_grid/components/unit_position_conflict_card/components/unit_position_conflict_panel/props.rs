use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_caption::ConflictCardCaptionProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_multi_stack::ConflictMultiStackProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_pair_row::ConflictPairRowProps;
use dioxus::prelude::*;

/// The position-collision card surface: the role-label caption over the pair-row and
/// multi-stack clash layouts.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionConflictPanelProps {
    pub caption: ConflictCardCaptionProps,
    pub pair_row: ConflictPairRowProps,
    pub multi_stack: ConflictMultiStackProps,
}
