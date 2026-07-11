use super::view::TileGlowIconView;
use dioxus::prelude::*;

/// The tile-glow look's props: the optional image source and its alt text. Built by
/// the `FramedIcon` dispatcher from `FramedIconModel`. Absent `source` draws the empty
/// framed square; a present `source` draws the covered image inside the frame.
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
