use dioxus::prelude::*;

/// The published `View` contract mirroring [`BurgerMenuGroupModel`](super::model::BurgerMenuGroupModel), threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BurgerMenuGroupView {
    pub on_close: EventHandler<MouseEvent>,
}

impl ddd::View for BurgerMenuGroupView {}
