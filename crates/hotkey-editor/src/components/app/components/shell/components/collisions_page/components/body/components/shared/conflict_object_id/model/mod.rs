use super::view::ConflictObjectIdView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct ConflictObjectIdModel {
    pub object_id: WarcraftObjectId,
}

impl From<&ConflictObjectIdView> for ConflictObjectIdModel {
    fn from(view: &ConflictObjectIdView) -> Self {
        let ConflictObjectIdView { object_id } = view.clone();
        Self { object_id }
    }
}

impl ddd::Model for ConflictObjectIdModel {
    type View = ConflictObjectIdView;
}
