use super::view::ActiveHeroLevelOptionView;
use dioxus::prelude::*;

/// The active hero-level option's props: its label and select handler.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveHeroLevelOptionProps {
    #[props(into)]
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ActiveHeroLevelOptionView> for ActiveHeroLevelOptionProps {
    fn from(view: &ActiveHeroLevelOptionView) -> Self {
        let ActiveHeroLevelOptionView { label, onclick } = view.clone();
        Self { label, onclick }
    }
}

impl ddd::Props for ActiveHeroLevelOptionProps {
    type View = ActiveHeroLevelOptionView;
}
