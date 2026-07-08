use super::super::super::props::HotkeyDetailHeaderProps;
use dioxus::prelude::*;

/// The text meta column of the detail-pane header: the unit name, its object id, and
/// the collision count.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyConflictMetaProps {
    pub name: String,
    pub unit_id_label: String,
    pub count: usize,
}

impl From<&HotkeyDetailHeaderProps> for HotkeyConflictMetaProps {
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
