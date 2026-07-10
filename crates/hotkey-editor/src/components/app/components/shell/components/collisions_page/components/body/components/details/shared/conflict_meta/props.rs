use super::view::ConflictMetaView;
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

impl From<&ConflictMetaView> for ConflictMetaProps {
    fn from(view: &ConflictMetaView) -> Self {
        let ConflictMetaView {
            name,
            unit_id,
            count,
        } = view.clone();
        Self {
            name,
            unit_id,
            count,
        }
    }
}

impl ddd::Props for ConflictMetaProps {
    type View = ConflictMetaView;
}
