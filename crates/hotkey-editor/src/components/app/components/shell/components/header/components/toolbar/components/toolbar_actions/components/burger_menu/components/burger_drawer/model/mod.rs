use super::view::BurgerDrawerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerDrawerModel {
    pub on_close: EventHandler<()>,
}

impl From<&BurgerDrawerView> for BurgerDrawerModel {
    fn from(view: &BurgerDrawerView) -> Self {
        let BurgerDrawerView { on_close } = view.clone();
        Self { on_close }
    }
}

impl ddd::Layered for BurgerDrawerModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for BurgerDrawerModel {
    type View = BurgerDrawerView;
}
