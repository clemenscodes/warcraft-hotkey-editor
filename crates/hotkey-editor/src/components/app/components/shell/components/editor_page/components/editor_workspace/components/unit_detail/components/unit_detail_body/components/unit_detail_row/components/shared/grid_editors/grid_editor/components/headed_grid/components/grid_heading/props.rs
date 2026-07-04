use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridHeadingProps {
    pub heading: &'static str,
}
