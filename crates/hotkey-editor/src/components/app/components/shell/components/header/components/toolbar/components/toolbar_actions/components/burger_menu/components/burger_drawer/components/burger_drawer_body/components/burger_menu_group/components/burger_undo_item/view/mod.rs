use dioxus::prelude::*;

/// The published `View` contract mirroring [`BurgerUndoItemModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BurgerUndoItemView {
    pub on_close: EventHandler<MouseEvent>,
}

impl ddd::View for BurgerUndoItemView {}
