use super::view::TileOverrideIdView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The database object id shown under the name.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideIdModel {
    pub object_id: WarcraftObjectId,
}

impl From<&TileOverrideIdView> for TileOverrideIdModel {
    fn from(view: &TileOverrideIdView) -> Self {
        let TileOverrideIdView { object_id } = view.clone();
        Self { object_id }
    }
}

impl ddd::Model for TileOverrideIdModel {
    type View = TileOverrideIdView;
}
