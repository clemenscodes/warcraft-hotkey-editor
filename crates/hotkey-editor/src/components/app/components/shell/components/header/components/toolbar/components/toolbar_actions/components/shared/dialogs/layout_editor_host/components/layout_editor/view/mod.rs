use dioxus::prelude::*;
use warcraft_keybinds::{GridCoordinate, GridLayout};

/// The published `View` contract mirroring [`LayoutEditorModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct LayoutEditorView {
    pub grid_layout: Signal<GridLayout>,
    pub editing_layout_tile: Signal<Option<GridCoordinate>>,
    pub dragging_layout_tile: Signal<Option<GridCoordinate>>,
    pub update_hotkeys_on_move: Signal<bool>,
    pub open: Signal<bool>,
}

impl ddd::View for LayoutEditorView {}
