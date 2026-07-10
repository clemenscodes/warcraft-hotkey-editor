use super::view::BreadcrumbCountView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbCountProps {
    pub count: usize,
}

impl From<&BreadcrumbCountView> for BreadcrumbCountProps {
    fn from(view: &BreadcrumbCountView) -> Self {
        let BreadcrumbCountView { count } = view.clone();
        Self { count }
    }
}

impl ddd::Props for BreadcrumbCountProps {
    type View = BreadcrumbCountView;
}
