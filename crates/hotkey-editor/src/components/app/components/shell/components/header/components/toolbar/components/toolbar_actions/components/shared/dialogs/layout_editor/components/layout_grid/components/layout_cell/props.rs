use super::state::LayoutCellState;
use dioxus::prelude::*;

/// One editable grid cell: its visual state, the letter it shows, its grid
/// address, and the drag/click handlers the editor wired for it.
#[derive(Props, Clone, PartialEq)]
pub struct LayoutCellProps {
    pub state: LayoutCellState,
    pub label: String,
    pub row: u8,
    pub column: u8,
    pub ondragstart: EventHandler<Event<DragData>>,
    pub ondragend: EventHandler<Event<DragData>>,
    pub ondragover: EventHandler<Event<DragData>>,
    pub ondrop: EventHandler<Event<DragData>>,
    pub onclick: EventHandler<MouseEvent>,
}
