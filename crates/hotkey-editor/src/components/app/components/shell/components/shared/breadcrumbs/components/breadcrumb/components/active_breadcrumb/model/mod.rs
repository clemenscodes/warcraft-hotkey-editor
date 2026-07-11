use super::view::ActiveBreadcrumbView;
use dioxus::prelude::*;

/// The active breadcrumb tab's props: its label, live count, and click handler.
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

impl ddd::Model for ActiveBreadcrumbModel {
    type View = ActiveBreadcrumbView;
}
