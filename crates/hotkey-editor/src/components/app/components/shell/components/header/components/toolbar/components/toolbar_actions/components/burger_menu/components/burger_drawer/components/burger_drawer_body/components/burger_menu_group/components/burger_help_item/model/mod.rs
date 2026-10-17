use super::view::BurgerHelpItemView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerHelpItemModel {
    pub on_close: EventHandler<()>,
}

impl From<&BurgerHelpItemView> for BurgerHelpItemModel {
    fn from(view: &BurgerHelpItemView) -> Self {
        let BurgerHelpItemView { on_close } = view.clone();
        Self { on_close }
    }
}

impl ddd::Layered for BurgerHelpItemModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for BurgerHelpItemModel {
    type View = BurgerHelpItemView;
}
