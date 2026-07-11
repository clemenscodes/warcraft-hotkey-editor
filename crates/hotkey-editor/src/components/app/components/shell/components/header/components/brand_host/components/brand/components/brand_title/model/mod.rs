use super::view::BrandTitleView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BrandTitleModel {
    pub title: &'static str,
}

impl From<&BrandTitleView> for BrandTitleModel {
    fn from(view: &BrandTitleView) -> Self {
        let BrandTitleView { title } = view.clone();
        Self { title }
    }
}

impl ddd::Model for BrandTitleModel {
    type View = BrandTitleView;
}
