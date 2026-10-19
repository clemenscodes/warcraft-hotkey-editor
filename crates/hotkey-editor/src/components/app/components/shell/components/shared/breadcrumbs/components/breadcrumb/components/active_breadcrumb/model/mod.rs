use super::view::ActiveBreadcrumbView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ActiveBreadcrumbModel {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ActiveBreadcrumbView> for ActiveBreadcrumbModel {
    fn from(view: &ActiveBreadcrumbView) -> Self {
        let ActiveBreadcrumbView {
            label,
            count,
            onclick,
        } = view.clone();
        Self {
            label,
            count,
            onclick,
        }
    }
}

impl ddd::Layered for ActiveBreadcrumbModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for ActiveBreadcrumbModel {
    type View = ActiveBreadcrumbView;
}
