use super::view::TileOverrideEmptyView;
use dioxus::prelude::*;

/// The prompt shown in the override panel before a tile is selected.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideEmptyProps {
    #[props(into)]
    pub message: String,
}

impl From<&TileOverrideEmptyView> for TileOverrideEmptyProps {
    fn from(view: &TileOverrideEmptyView) -> Self {
        let TileOverrideEmptyView { message } = view.clone();
        Self { message }
    }
}

impl ddd::Props for TileOverrideEmptyProps {
    type View = TileOverrideEmptyView;
}
