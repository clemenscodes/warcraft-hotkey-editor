use dioxus::prelude::*;

/// The resolved image source and its alt text.
#[derive(Props, Clone, PartialEq)]
pub struct FramedIconImageProps {
    #[props(into)]
    pub source: String,
    #[props(into)]
    pub alt: String,
}
