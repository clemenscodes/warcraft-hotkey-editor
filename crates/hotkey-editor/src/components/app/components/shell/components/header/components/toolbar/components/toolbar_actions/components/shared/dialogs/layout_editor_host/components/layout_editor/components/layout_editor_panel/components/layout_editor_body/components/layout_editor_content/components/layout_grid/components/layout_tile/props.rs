use super::state::LayoutTileState;
use super::view::LayoutTileView;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// One editable grid cell: its visual state, the letter it shows, its grid
/// address, and the drag/click handlers the editor wired for it.
#[derive(Props, Clone, PartialEq)]
pub struct LayoutTileProps {
    pub state: LayoutTileState,
    pub label: String,
    pub coordinate: GridCoordinate,
    pub ondragstart: EventHandler<Event<DragData>>,
    pub ondragend: EventHandler<Event<DragData>>,
    pub ondragover: EventHandler<Event<DragData>>,
    pub ondrop: EventHandler<Event<DragData>>,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&LayoutTileView> for LayoutTileProps {
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

impl ddd::Props for LayoutTileProps {
    type View = LayoutTileView;
}
