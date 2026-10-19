use super::view::BurgerDrawerBodyView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerDrawerBodyModel {
    pub on_close: EventHandler<()>,
}

impl From<&BurgerDrawerBodyView> for BurgerDrawerBodyModel {
    fn from(view: &BurgerDrawerBodyView) -> Self {
        let BurgerDrawerBodyView { on_close } = view.clone();
        Self { on_close }
    }
}

impl ddd::Layered for BurgerDrawerBodyModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for BurgerDrawerBodyModel {
    type View = BurgerDrawerBodyView;
}
