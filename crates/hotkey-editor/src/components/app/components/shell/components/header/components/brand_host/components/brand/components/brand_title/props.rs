use super::view::BrandTitleView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BrandTitleProps {
    pub title: &'static str,
}

impl From<&BrandTitleView> for BrandTitleProps {
    fn from(view: &BrandTitleView) -> Self {
        let BrandTitleView { title } = view.clone();
        Self { title }
    }
}

impl ddd::Props for BrandTitleProps {
    type View = BrandTitleView;
}
