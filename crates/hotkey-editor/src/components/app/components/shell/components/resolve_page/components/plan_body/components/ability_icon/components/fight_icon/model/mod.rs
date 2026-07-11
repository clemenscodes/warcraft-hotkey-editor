use super::view::FightIconView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct FightIconModel {
    pub src: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&FightIconView> for FightIconModel {
    fn from(view: &FightIconView) -> Self {
        let FightIconView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Model for FightIconModel {
    type View = FightIconView;
}
