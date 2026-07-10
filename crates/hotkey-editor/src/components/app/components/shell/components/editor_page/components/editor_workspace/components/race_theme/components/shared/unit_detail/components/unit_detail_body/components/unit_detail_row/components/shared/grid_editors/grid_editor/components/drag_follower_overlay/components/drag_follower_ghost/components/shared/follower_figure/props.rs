use super::view::FollowerFigureView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FollowerFigureProps {
    /// The dragged tile's icon. A follower always has one; you cannot drag a tile
    /// without an icon.
    pub src: String,
    pub alt: String,
}

impl From<&FollowerFigureView> for FollowerFigureProps {
    fn from(view: &FollowerFigureView) -> Self {
        let FollowerFigureView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Props for FollowerFigureProps {
    type View = FollowerFigureView;
}
