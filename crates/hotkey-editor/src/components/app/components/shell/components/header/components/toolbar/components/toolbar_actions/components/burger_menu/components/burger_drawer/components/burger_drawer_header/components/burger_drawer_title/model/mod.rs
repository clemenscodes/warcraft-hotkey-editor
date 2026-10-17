use super::view::BurgerDrawerTitleView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerDrawerTitleModel {
    pub title: &'static str,
}

impl From<&BurgerDrawerTitleView> for BurgerDrawerTitleModel {
    fn from(view: &BurgerDrawerTitleView) -> Self {
        let BurgerDrawerTitleView { title } = view.clone();
        Self { title }
    }
}

impl ddd::Layered for BurgerDrawerTitleModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for BurgerDrawerTitleModel {
    type View = BurgerDrawerTitleView;
}
