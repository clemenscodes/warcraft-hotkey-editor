use super::view::BurgerDrawerBodyView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerDrawerBodyModel {
    pub on_close: EventHandler<MouseEvent>,
}

impl From<&BurgerDrawerBodyView> for BurgerDrawerBodyModel {
    fn from(view: &BurgerDrawerBodyView) -> Self {
        let BurgerDrawerBodyView { on_close } = view.clone();
        Self { on_close }
    }
}

impl ddd::Model for BurgerDrawerBodyModel {
    type View = BurgerDrawerBodyView;
}
