use dioxus::prelude::*;

use super::super::super::logic::FollowerPresentation;

#[derive(Props, Clone, PartialEq)]
pub struct FollowerFigureProps {
    /// The dragged tile's icon. A follower always has one; you cannot drag a tile
    /// without an icon.
    pub src: String,
    pub alt: String,
}

impl From<&FollowerPresentation> for FollowerFigureProps {
    fn from(presentation: &FollowerPresentation) -> Self {
        let src = presentation.src.clone();
        let alt = presentation.alt.clone();
        Self { src, alt }
    }
}
