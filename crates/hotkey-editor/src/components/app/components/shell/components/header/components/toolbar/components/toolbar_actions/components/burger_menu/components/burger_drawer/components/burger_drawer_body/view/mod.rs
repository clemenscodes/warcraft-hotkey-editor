use dioxus::prelude::*;

/// The published `View` contract mirroring [`BurgerDrawerBodyModel`](super::model::BurgerDrawerBodyModel), threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BurgerDrawerBodyView {
    pub on_close: EventHandler<MouseEvent>,
}

impl ddd::View for BurgerDrawerBodyView {}
