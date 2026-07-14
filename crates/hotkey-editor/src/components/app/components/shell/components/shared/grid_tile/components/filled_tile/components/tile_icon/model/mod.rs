use super::view::TileIconView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TileIconModel {
    pub src: Option<String>,
    pub alt: String,
}

impl From<&TileIconView> for TileIconModel {
    fn from(view: &TileIconView) -> Self {
        let TileIconView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Model for TileIconModel {
    type View = TileIconView;
}
