use super::view::FollowerIconView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FollowerIconModel {
    /// The dragged tile's icon. A follower always has one; you cannot drag a tile
    /// without an icon.
    pub src: String,
    pub alt: String,
}

impl From<&FollowerIconView> for FollowerIconModel {
    fn from(view: &FollowerIconView) -> Self {
        let FollowerIconView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Model for FollowerIconModel {
    type View = FollowerIconView;
}
