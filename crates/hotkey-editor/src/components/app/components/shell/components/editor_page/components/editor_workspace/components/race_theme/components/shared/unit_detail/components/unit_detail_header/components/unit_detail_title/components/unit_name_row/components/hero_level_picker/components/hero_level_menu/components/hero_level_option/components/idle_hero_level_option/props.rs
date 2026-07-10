use super::view::IdleHeroLevelOptionView;
use dioxus::prelude::*;

/// The idle hero-level option's props: its label and select handler.
#[derive(Props, Clone, PartialEq)]
pub struct IdleHeroLevelOptionProps {
    #[props(into)]
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&IdleHeroLevelOptionView> for IdleHeroLevelOptionProps {
    fn from(view: &IdleHeroLevelOptionView) -> Self {
        let IdleHeroLevelOptionView { label, onclick } = view.clone();
        Self { label, onclick }
    }
}

impl ddd::Props for IdleHeroLevelOptionProps {
    type View = IdleHeroLevelOptionView;
}
