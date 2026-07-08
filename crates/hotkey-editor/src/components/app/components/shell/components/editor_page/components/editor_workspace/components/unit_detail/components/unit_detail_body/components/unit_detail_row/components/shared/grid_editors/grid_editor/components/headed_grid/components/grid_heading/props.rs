use crate::components::app::components::shell::components::shared::gold_heading::{
    GoldHeadingProps, GoldHeadingVariant,
};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridHeadingProps {
    pub heading: &'static str,
}

impl From<&GridHeadingProps> for GoldHeadingProps {
    fn from(props: &GridHeadingProps) -> Self {
        let title = String::from(props.heading);
        let variant = GoldHeadingVariant::Grid;
        Self { title, variant }
    }
}
