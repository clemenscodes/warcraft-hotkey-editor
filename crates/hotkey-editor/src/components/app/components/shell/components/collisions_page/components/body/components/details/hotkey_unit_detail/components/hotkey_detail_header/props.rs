use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_detail_unit::ConflictDetailUnitProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_meta::ConflictMetaProps;
use dioxus::prelude::*;

/// The detail-pane header row: the selected unit button and its text meta column.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyDetailHeaderProps {
    pub unit: ConflictDetailUnitProps,
    pub name: String,
    pub unit_id_label: String,
    pub count: usize,
}

impl From<&HotkeyDetailHeaderProps> for ConflictMetaProps {
    fn from(props: &HotkeyDetailHeaderProps) -> Self {
        let name = props.name.clone();
        let unit_id_label = props.unit_id_label.clone();
        let count = props.count;
        Self {
            name,
            unit_id_label,
            count,
        }
    }
}
