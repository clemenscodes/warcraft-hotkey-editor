use dioxus::prelude::*;

/// The prompt shown in the override panel before a tile is selected.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideEmptyProps {
    #[props(into)]
    pub message: String,
}
