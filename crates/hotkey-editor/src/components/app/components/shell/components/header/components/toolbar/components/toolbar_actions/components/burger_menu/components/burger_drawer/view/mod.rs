use dioxus::prelude::*;

/// The published `View` contract mirroring [`BurgerDrawerModel`](super::model::BurgerDrawerModel), threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BurgerDrawerView {
    pub on_close: EventHandler<MouseEvent>,
}

impl ddd::View for BurgerDrawerView {}
