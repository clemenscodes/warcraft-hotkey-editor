use super::super::super::GridTileState;
use super::view::FilledTileView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FilledTileModel {
    pub state: GridTileState,
    pub icon: Option<String>,
    pub label: String,
    pub is_dragging_source: bool,
    pub is_drag_over: bool,
}

impl From<&FilledTileView> for FilledTileModel {
    fn from(view: &FilledTileView) -> Self {
        let FilledTileView {
            state,
            icon,
            label,
            is_dragging_source,
            is_drag_over,
        } = view.clone();
        Self {
            state,
            icon,
            label,
            is_dragging_source,
            is_drag_over,
        }
    }
}

impl ddd::Model for FilledTileModel {
    type View = FilledTileView;
}
