use super::super::super::UnitPositionDetailHeaderProps;
use dioxus::prelude::*;

/// The text meta column of the position-collision detail header: the unit's name,
/// object id, and collision count.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionConflictMetaProps {
    pub name: String,
    pub unit_id_label: String,
    pub count: usize,
}

impl From<&UnitPositionDetailHeaderProps> for UnitPositionConflictMetaProps {
    fn from(props: &UnitPositionDetailHeaderProps) -> Self {
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
