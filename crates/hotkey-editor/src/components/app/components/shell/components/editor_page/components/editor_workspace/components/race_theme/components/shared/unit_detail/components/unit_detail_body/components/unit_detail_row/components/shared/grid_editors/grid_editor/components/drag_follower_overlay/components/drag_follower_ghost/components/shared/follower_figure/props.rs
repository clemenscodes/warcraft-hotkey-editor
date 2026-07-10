use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FollowerFigureProps {
    /// The dragged tile's icon. A follower always has one; you cannot drag a tile
    /// without an icon.
    pub src: String,
    pub alt: String,
}
