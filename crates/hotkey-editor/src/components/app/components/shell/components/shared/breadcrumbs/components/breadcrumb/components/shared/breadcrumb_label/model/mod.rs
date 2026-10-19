use super::view::BreadcrumbLabelView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbLabelModel {
    #[props(into)]
    pub text: String,
}

impl From<&BreadcrumbLabelView> for BreadcrumbLabelModel {
    fn from(view: &BreadcrumbLabelView) -> Self {
        let BreadcrumbLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Layered for BreadcrumbLabelModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for BreadcrumbLabelModel {
    type View = BreadcrumbLabelView;
}
