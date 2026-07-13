use super::state::LayoutTileState;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// The published `View` contract mirroring [`LayoutTileModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct LayoutTileView {
    pub state: LayoutTileState,
    pub label: String,
    pub coordinate: GridCoordinate,
    pub ondragstart: EventHandler<Event<DragData>>,
    pub ondragend: EventHandler<Event<DragData>>,
    pub ondragover: EventHandler<Event<DragData>>,
    pub ondrop: EventHandler<Event<DragData>>,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for LayoutTileView {}
