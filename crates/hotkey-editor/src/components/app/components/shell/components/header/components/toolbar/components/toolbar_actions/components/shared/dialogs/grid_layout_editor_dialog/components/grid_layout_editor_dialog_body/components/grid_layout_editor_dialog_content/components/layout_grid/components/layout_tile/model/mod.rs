use super::state::LayoutTileState;
use super::view::LayoutTileView;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

#[derive(Props, Clone, PartialEq)]
pub struct LayoutTileModel {
    pub state: LayoutTileState,
    pub label: String,
    pub coordinate: GridCoordinate,
    pub ondragstart: EventHandler<Event<DragData>>,
    pub ondragend: EventHandler<Event<DragData>>,
    pub ondragover: EventHandler<Event<DragData>>,
    pub ondrop: EventHandler<Event<DragData>>,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&LayoutTileView> for LayoutTileModel {
    fn from(view: &LayoutTileView) -> Self {
        let LayoutTileView {
            state,
            label,
            coordinate,
            ondragstart,
            ondragend,
            ondragover,
            ondrop,
            onclick,
        } = view.clone();
        Self {
            state,
            label,
            coordinate,
            ondragstart,
            ondragend,
            ondragover,
            ondrop,
            onclick,
        }
    }
}

impl ddd::Model for LayoutTileModel {
    type View = LayoutTileView;
}
