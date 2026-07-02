use dioxus::prelude::*;

/// A stat column's icon image: the asset source and its alt text.
#[derive(Props, Clone, PartialEq)]
pub struct StatIconImgProps {
    pub src: Asset,
    #[props(into)]
    pub alt: String,
}
