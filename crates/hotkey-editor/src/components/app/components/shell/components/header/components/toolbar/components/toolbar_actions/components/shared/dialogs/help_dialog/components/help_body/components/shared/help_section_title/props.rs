use crate::components::app::components::shell::components::shared::gold_heading::{
    GoldHeadingProps, GoldHeadingVariant,
};
use dioxus::prelude::*;

/// The section title's only input: the heading text.
#[derive(Props, Clone, PartialEq)]
pub struct HelpSectionTitleProps {
    #[props(into)]
    pub title: String,
}

impl From<&HelpSectionTitleProps> for GoldHeadingProps {
    fn from(props: &HelpSectionTitleProps) -> Self {
        let title = props.title.clone();
        let variant = GoldHeadingVariant::Section;
        Self { title, variant }
    }
}
