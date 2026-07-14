use super::view::IdleBreadcrumbView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IdleBreadcrumbModel {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&IdleBreadcrumbView> for IdleBreadcrumbModel {
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

impl ddd::Model for IdleBreadcrumbModel {
    type View = IdleBreadcrumbView;
}
