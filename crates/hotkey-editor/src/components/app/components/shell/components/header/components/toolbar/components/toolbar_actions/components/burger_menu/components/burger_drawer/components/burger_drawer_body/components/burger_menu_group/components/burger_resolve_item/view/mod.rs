use dioxus::prelude::*;

/// The published `View` contract mirroring [`BurgerResolveItemModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BurgerResolveItemView {
    pub on_close: EventHandler<MouseEvent>,
}

impl ddd::View for BurgerResolveItemView {}
