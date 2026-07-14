use super::view::CardGlowIconView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CardGlowIconModel {
    pub source: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&CardGlowIconView> for CardGlowIconModel {
    fn from(view: &CardGlowIconView) -> Self {
        let CardGlowIconView { source, alt } = view.clone();
        Self { source, alt }
    }
}

impl ddd::Model for CardGlowIconModel {
    type View = CardGlowIconView;
}
