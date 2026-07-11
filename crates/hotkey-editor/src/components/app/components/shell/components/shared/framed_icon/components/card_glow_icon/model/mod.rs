use super::view::CardGlowIconView;
use dioxus::prelude::*;

/// The card-glow look's props: the optional image source and its alt text. Built by
/// the `FramedIcon` dispatcher from `FramedIconModel`. Absent `source` draws the empty
/// framed square; a present `source` draws the covered image inside the frame.
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
