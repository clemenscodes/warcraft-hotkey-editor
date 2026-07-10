use super::view::ConflictObjectIdView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// A database object id (unit or ability), shown as a monospace caption on a
/// collision card.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictObjectIdProps {
    pub object_id: WarcraftObjectId,
}

impl From<&ConflictObjectIdView> for ConflictObjectIdProps {
    fn from(view: &ConflictObjectIdView) -> Self {
        let ConflictObjectIdView { object_id } = view.clone();
        Self { object_id }
    }
}

impl ddd::Props for ConflictObjectIdProps {
    type View = ConflictObjectIdView;
}
