use super::view::HeroLevelOptionView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HeroLevelOptionModel {
    pub level_index: u32,
    pub on_close: EventHandler<()>,
}

impl From<&HeroLevelOptionView> for HeroLevelOptionModel {
    fn from(view: &HeroLevelOptionView) -> Self {
        let HeroLevelOptionView {
            level_index,
            on_close,
        } = view.clone();
        Self {
            level_index,
            on_close,
        }
    }
}

impl ddd::Layered for HeroLevelOptionModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for HeroLevelOptionModel {
    type View = HeroLevelOptionView;
}
