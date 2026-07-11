use super::view::TilePlainIconView;
use dioxus::prelude::*;

/// The tile-plain look's props: the optional image source and its alt text. Built by
/// the `FramedIcon` dispatcher from `FramedIconModel`. Absent `source` draws the empty
/// framed square; a present `source` draws the covered image inside the frame.
#[derive(Props, Clone, PartialEq)]
pub struct TilePlainIconModel {
    pub source: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&TilePlainIconView> for TilePlainIconModel {
    fn from(view: &TilePlainIconView) -> Self {
        let TilePlainIconView { source, alt } = view.clone();
        Self { source, alt }
    }
}

impl ddd::Model for TilePlainIconModel {
    type View = TilePlainIconView;
}
