use dioxus::prelude::*;

/// The published `View` contract mirroring [`BurgerDrawerHeaderModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BurgerDrawerHeaderView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for BurgerDrawerHeaderView {}
