use super::view::ConflictMetaView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct ConflictMetaModel {
    pub name: String,
    pub unit_id: WarcraftObjectId,
    pub count: usize,
}

impl From<&ConflictMetaView> for ConflictMetaModel {
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

impl ddd::Model for ConflictMetaModel {
    type View = ConflictMetaView;
}
