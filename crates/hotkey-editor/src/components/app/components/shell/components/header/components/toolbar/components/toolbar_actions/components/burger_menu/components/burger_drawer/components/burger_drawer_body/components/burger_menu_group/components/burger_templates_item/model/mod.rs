use super::view::BurgerTemplatesItemView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerTemplatesItemModel {
    pub on_close: EventHandler<()>,
}

impl From<&BurgerTemplatesItemView> for BurgerTemplatesItemModel {
    fn from(view: &BurgerTemplatesItemView) -> Self {
        let BurgerTemplatesItemView { on_close } = view.clone();
        Self { on_close }
    }
}

impl ddd::Layered for BurgerTemplatesItemModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for BurgerTemplatesItemModel {
    type View = BurgerTemplatesItemView;
}
