use super::view::FightIconView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct FightIconProps {
    pub src: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&FightIconView> for FightIconProps {
    fn from(view: &FightIconView) -> Self {
        let FightIconView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Props for FightIconProps {
    type View = FightIconView;
}
