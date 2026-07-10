use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The header text column owns the name heading and the id line beneath it.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideHeaderTextProps {
    pub name_text: String,
    pub object_id: WarcraftObjectId,
}
