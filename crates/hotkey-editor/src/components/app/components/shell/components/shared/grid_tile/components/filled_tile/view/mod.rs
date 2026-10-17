use super::super::super::GridTileState;

#[derive(Clone, PartialEq)]
pub struct FilledTileView {
    pub state: GridTileState,
    pub icon: Option<String>,
    pub label: String,
    pub is_dragging_source: bool,
    pub is_drag_over: bool,
}

impl ddd::Layered for FilledTileView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for FilledTileView {}
