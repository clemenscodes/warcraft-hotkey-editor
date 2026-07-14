use super::view::TileGlowIconView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TileGlowIconModel {
    pub source: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&TileGlowIconView> for TileGlowIconModel {
    fn from(view: &TileGlowIconView) -> Self {
        let TileGlowIconView { source, alt } = view.clone();
        Self { source, alt }
    }
}

impl ddd::Model for TileGlowIconModel {
    type View = TileGlowIconView;
}
