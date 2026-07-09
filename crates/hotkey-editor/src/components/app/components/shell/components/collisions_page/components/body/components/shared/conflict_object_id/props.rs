use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// A database object id (unit or ability), shown as a monospace caption on a
/// collision card.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictObjectIdProps {
    pub object_id: WarcraftObjectId,
}
