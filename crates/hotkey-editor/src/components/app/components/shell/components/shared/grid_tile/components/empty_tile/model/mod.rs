use super::super::super::GridTileState;
use super::view::EmptyTileView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct EmptyTileModel {
    pub state: GridTileState,
    pub is_drag_over: bool,
}

impl From<&EmptyTileView> for EmptyTileModel {
    fn from(view: &EmptyTileView) -> Self {
        let EmptyTileView {
            state,
            is_drag_over,
        } = view.clone();
        Self {
            state,
            is_drag_over,
        }
    }
}

impl ddd::Model for EmptyTileModel {
    type View = EmptyTileView;
}
