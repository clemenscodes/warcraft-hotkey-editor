use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridCoordinate;

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

#[derive(Clone, PartialEq)]
pub struct CollisionCardData {
    pub is_selected: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub count: usize,
    pub content: CollisionCardContent,
}
