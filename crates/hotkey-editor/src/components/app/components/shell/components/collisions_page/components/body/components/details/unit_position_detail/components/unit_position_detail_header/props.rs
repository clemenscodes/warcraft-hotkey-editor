use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_detail_unit::ConflictDetailUnitProps;
use dioxus::prelude::*;

/// The position-collision detail pane header: the selected unit's icon button beside
/// its name, object id, and collision count.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionDetailHeaderProps {
    pub unit: ConflictDetailUnitProps,
    pub name: String,
    pub unit_id_label: String,
    pub count: usize,
}
