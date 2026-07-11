use super::view::FramedIconImageView;
use dioxus::prelude::*;

/// The resolved image source and its alt text.
#[derive(Props, Clone, PartialEq)]
pub struct FramedIconImageModel {
    #[props(into)]
    pub source: String,
    #[props(into)]
    pub alt: String,
}

impl From<&FramedIconImageView> for FramedIconImageModel {
    fn from(view: &FramedIconImageView) -> Self {
        let FramedIconImageView { source, alt } = view.clone();
        Self { source, alt }
    }
}

impl ddd::Model for FramedIconImageModel {
    type View = FramedIconImageView;
}
