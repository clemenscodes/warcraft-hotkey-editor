use dioxus::prelude::*;

/// The text meta column of a detail-pane header: the unit name, its object id, and the
/// collision count.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictMetaProps {
    pub name: String,
    pub unit_id_label: String,
    pub count: usize,
}
