use super::view::TileOverrideNameView;
use dioxus::prelude::*;

/// The active ability / unit name shown in the override panel header.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideNameModel {
    #[props(into)]
    pub text: String,
}

impl From<&TileOverrideNameView> for TileOverrideNameModel {
    fn from(view: &TileOverrideNameView) -> Self {
        let TileOverrideNameView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for TileOverrideNameModel {
    type View = TileOverrideNameView;
}
