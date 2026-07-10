use super::view::ActiveBreadcrumbView;
use dioxus::prelude::*;

/// The active breadcrumb tab's props: its label, live count, and click handler.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveBreadcrumbProps {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ActiveBreadcrumbView> for ActiveBreadcrumbProps {
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

impl ddd::Props for ActiveBreadcrumbProps {
    type View = ActiveBreadcrumbView;
}
