use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The text meta column of a detail-pane header: the unit name, its object id, and the
/// collision count.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictMetaProps {
    pub name: String,
    pub unit_id: WarcraftObjectId,
    pub count: usize,
}
