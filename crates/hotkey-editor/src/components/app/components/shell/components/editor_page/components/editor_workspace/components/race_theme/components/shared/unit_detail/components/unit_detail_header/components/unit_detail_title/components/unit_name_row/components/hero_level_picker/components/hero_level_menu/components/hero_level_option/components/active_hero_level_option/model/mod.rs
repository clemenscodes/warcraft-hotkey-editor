use super::view::ActiveHeroLevelOptionView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ActiveHeroLevelOptionModel {
    #[props(into)]
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ActiveHeroLevelOptionView> for ActiveHeroLevelOptionModel {
    fn from(view: &ActiveHeroLevelOptionView) -> Self {
        let ActiveHeroLevelOptionView { label, onclick } = view.clone();
        Self { label, onclick }
    }
}

impl ddd::Model for ActiveHeroLevelOptionModel {
    type View = ActiveHeroLevelOptionView;
}
