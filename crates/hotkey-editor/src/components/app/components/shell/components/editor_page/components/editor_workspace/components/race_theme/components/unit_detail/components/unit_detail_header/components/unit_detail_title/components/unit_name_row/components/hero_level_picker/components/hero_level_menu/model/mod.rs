use super::view::HeroLevelMenuView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HeroLevelMenuModel {
    pub on_close: EventHandler<()>,
}

impl From<&HeroLevelMenuView> for HeroLevelMenuModel {
    fn from(view: &HeroLevelMenuView) -> Self {
        let HeroLevelMenuView { on_close } = view.clone();
        Self { on_close }
    }
}

impl ddd::Layered for HeroLevelMenuModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for HeroLevelMenuModel {
    type View = HeroLevelMenuView;
}
