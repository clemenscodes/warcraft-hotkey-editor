use dioxus::prelude::*;

/// The published `View` contract mirroring [`BurgerCloseModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BurgerCloseView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for BurgerCloseView {}
