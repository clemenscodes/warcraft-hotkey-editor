use dioxus::prelude::*;

/// The published `View` contract mirroring [`BurgerRedoItemModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BurgerRedoItemView {
    pub on_close: EventHandler<MouseEvent>,
}

impl ddd::View for BurgerRedoItemView {}
