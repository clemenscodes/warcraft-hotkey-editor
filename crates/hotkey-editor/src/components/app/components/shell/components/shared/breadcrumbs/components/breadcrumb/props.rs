use super::view::BreadcrumbView;
use dioxus::prelude::*;

/// One breadcrumb tab: its label, live count, active flag, and the navigation
/// handler it runs when clicked.
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbProps {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&BreadcrumbView> for BreadcrumbProps {
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

impl ddd::Props for BreadcrumbProps {
    type View = BreadcrumbView;
}
