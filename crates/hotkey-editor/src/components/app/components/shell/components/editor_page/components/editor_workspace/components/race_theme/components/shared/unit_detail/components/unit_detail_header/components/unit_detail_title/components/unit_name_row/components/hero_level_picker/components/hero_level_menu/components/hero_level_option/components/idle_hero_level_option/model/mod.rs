use super::view::IdleHeroLevelOptionView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IdleHeroLevelOptionModel {
    #[props(into)]
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&IdleHeroLevelOptionView> for IdleHeroLevelOptionModel {
    fn from(view: &IdleHeroLevelOptionView) -> Self {
        let IdleHeroLevelOptionView { label, onclick } = view.clone();
        Self { label, onclick }
    }
}

impl ddd::Model for IdleHeroLevelOptionModel {
    type View = IdleHeroLevelOptionView;
}
