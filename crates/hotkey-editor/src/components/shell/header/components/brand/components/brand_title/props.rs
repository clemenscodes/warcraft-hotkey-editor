use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BrandTitleProps {
    pub title: &'static str,
}
