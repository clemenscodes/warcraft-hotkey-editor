use super::view::TileOverrideHeaderTextView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The header text column owns the name heading and the id line beneath it.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideHeaderTextModel {
    pub name_text: String,
    pub object_id: WarcraftObjectId,
}

impl From<&TileOverrideHeaderTextView> for TileOverrideHeaderTextModel {
    fn from(view: &TileOverrideHeaderTextView) -> Self {
        let TileOverrideHeaderTextView {
            name_text,
            object_id,
        } = view.clone();
        Self {
            name_text,
            object_id,
        }
    }
}

impl ddd::Model for TileOverrideHeaderTextModel {
    type View = TileOverrideHeaderTextView;
}
