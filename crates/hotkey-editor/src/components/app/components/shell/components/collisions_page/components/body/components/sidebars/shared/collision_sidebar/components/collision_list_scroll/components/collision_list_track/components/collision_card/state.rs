use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridCoordinate;

/// What a collision card shows. A unit card carries a portrait, name, and object
/// id; a cross-unit island card carries the highlighted grid coordinate. Both also
/// carry a collision count, kept beside this on [`CollisionCardProps`](super::props).
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
