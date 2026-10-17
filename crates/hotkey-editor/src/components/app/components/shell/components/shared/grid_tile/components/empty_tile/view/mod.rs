use super::super::super::GridTileState;

#[derive(Clone, PartialEq)]
pub struct EmptyTileView {
    pub state: GridTileState,
    pub is_drag_over: bool,
}

impl ddd::Layered for EmptyTileView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for EmptyTileView {}
