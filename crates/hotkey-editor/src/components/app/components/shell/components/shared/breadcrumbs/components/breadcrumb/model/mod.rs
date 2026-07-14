use super::view::BreadcrumbView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbModel {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&BreadcrumbView> for BreadcrumbModel {
    fn from(view: &BreadcrumbView) -> Self {
        let BreadcrumbView {
            label,
            count,
            active,
            onclick,
        } = view.clone();
        Self {
            label,
            count,
            active,
            onclick,
        }
    }
}

impl ddd::Model for BreadcrumbModel {
    type View = BreadcrumbView;
}
