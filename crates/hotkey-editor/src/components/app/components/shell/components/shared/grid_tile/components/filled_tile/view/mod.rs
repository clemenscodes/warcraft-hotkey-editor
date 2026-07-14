use super::super::super::GridTileState;

#[derive(Clone, PartialEq)]
pub struct FilledTileView {
    pub state: GridTileState,
    pub icon: Option<String>,
    pub label: String,
    pub is_dragging_source: bool,
    pub is_drag_over: bool,
}

impl ddd::View for FilledTileView {}
