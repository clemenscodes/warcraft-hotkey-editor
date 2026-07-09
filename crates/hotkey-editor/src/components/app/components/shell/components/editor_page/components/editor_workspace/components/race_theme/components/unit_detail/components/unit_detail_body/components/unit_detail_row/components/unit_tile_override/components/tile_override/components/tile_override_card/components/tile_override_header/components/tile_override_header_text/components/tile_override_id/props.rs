use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The database object id shown under the name.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideIdProps {
    pub object_id: WarcraftObjectId,
}
