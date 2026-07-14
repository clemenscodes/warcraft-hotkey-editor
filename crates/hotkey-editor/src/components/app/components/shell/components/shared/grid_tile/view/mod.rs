use super::state::GridTileState;
use dioxus::prelude::*;
use warcraft_keybinds::{ColumnIndex, GridCoordinate, RowIndex};

#[derive(Props, Clone, PartialEq)]
pub struct GridTileView {
    #[props(default = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero))]
    pub coordinate: GridCoordinate,
    #[props(default)]
    pub icon: Option<String>,
    #[props(default)]
    pub label: String,
    #[props(default)]
    pub state: GridTileState,
    #[props(default)]
    pub is_dragging_source: bool,
    #[props(default)]
    pub is_drag_over: bool,
}

impl ddd::View for GridTileView {}
