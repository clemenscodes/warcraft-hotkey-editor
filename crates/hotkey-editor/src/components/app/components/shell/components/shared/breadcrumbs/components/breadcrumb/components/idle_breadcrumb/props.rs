use super::view::IdleBreadcrumbView;
use dioxus::prelude::*;

/// The idle breadcrumb tab's props: its label, live count, and click handler.
#[derive(Props, Clone, PartialEq)]
pub struct IdleBreadcrumbProps {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&IdleBreadcrumbView> for IdleBreadcrumbProps {
    fn from(view: &IdleBreadcrumbView) -> Self {
        let IdleBreadcrumbView {
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

impl ddd::Props for IdleBreadcrumbProps {
    type View = IdleBreadcrumbView;
}
