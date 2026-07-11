use super::view::BreadcrumbCountView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbCountModel {
    pub count: usize,
}

impl From<&BreadcrumbCountView> for BreadcrumbCountModel {
    fn from(view: &BreadcrumbCountView) -> Self {
        let BreadcrumbCountView { count } = view.clone();
        Self { count }
    }
}

impl ddd::Model for BreadcrumbCountModel {
    type View = BreadcrumbCountView;
}
