use crate::components::app::components::shell::components::shared::gold_heading::{
    GoldHeadingProps, GoldHeadingVariant,
};
use dioxus::prelude::*;

/// The title's only input: the heading text.
#[derive(Props, Clone, PartialEq)]
pub struct DialogTitleProps {
    #[props(into)]
    pub title: String,
}

impl From<&DialogTitleProps> for GoldHeadingProps {
    fn from(props: &DialogTitleProps) -> Self {
        let title = props.title.clone();
        let variant = GoldHeadingVariant::Dialog;
        Self { title, variant }
    }
}
