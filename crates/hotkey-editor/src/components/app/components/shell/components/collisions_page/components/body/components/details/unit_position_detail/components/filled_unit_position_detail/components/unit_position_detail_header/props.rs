use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_detail_unit::ConflictDetailUnitProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_meta::ConflictMetaProps;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The position-collision detail pane header: the selected unit's icon button beside
/// its name, object id, and collision count.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionDetailHeaderProps {
    pub unit: ConflictDetailUnitProps,
    pub name: String,
    pub unit_id: WarcraftObjectId,
    pub count: usize,
}

impl From<&UnitPositionDetailHeaderProps> for ConflictMetaProps {
    fn from(props: &UnitPositionDetailHeaderProps) -> Self {
        let name = props.name.clone();
        let unit_id = props.unit_id;
        let count = props.count;
        Self {
            name,
            unit_id,
            count,
        }
    }
}
