use super::view::BreadcrumbLabelView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbLabelProps {
    #[props(into)]
    pub text: String,
}

impl From<&BreadcrumbLabelView> for BreadcrumbLabelProps {
    fn from(view: &BreadcrumbLabelView) -> Self {
        let BreadcrumbLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for BreadcrumbLabelProps {
    type View = BreadcrumbLabelView;
}
