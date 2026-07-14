use super::view::HeroLevelBackdropView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HeroLevelBackdropModel {
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&HeroLevelBackdropView> for HeroLevelBackdropModel {
    fn from(view: &HeroLevelBackdropView) -> Self {
        let HeroLevelBackdropView { onclick } = view.clone();
        Self { onclick }
    }
}

impl ddd::Model for HeroLevelBackdropModel {
    type View = HeroLevelBackdropView;
}
