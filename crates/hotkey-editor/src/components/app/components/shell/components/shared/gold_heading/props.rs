use super::state::GoldHeadingVariant;
use dioxus::prelude::*;

/// The shared gold heading's inputs: the heading text and which weight of the
/// look to wear.
#[derive(Props, Clone, PartialEq)]
pub struct GoldHeadingProps {
    #[props(into)]
    pub title: String,
    pub variant: GoldHeadingVariant,
}
