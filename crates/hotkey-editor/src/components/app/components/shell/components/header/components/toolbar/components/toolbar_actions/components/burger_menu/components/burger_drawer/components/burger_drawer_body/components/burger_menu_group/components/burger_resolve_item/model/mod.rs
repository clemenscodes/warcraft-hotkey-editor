use super::view::BurgerResolveItemView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerResolveItemModel {
    pub on_close: EventHandler<()>,
}

impl From<&BurgerResolveItemView> for BurgerResolveItemModel {
    fn from(view: &BurgerResolveItemView) -> Self {
        let BurgerResolveItemView { on_close } = view.clone();
        Self { on_close }
    }
}

impl ddd::Layered for BurgerResolveItemModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for BurgerResolveItemModel {
    type View = BurgerResolveItemView;
}
