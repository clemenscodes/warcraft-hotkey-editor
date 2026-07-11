use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridCoordinate;

/// What a collision card shows. A unit card carries a portrait, name, and object
/// id; a cross-unit island card carries the highlighted grid coordinate. Both also
/// carry a collision count, kept beside this on [`CollisionCardData`].
#[derive(Clone, PartialEq)]
pub enum CollisionCardContent {
    Unit {
        icon_url: Option<String>,
        name: String,
        unit_id: WarcraftObjectId,
    },
    Island {
        coordinate: GridCoordinate,
    },
}

/// One collision-sidebar card's shaped data: its selected state, click handler, live
/// collision count, and the content that fills it. Built per sidebar (island / unit
/// cards) and threaded down through the shared collision sidebar to the track, which
/// builds each `CollisionCard` from these fields.
#[derive(Clone, PartialEq)]
pub struct CollisionCardData {
    pub is_selected: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub count: usize,
    pub content: CollisionCardContent,
}
