use super::view::ActiveResolveSectionTabView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ActiveResolveSectionTabModel {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ActiveResolveSectionTabView> for ActiveResolveSectionTabModel {
    fn from(view: &ActiveResolveSectionTabView) -> Self {
        let ActiveResolveSectionTabView {
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

impl ddd::Model for ActiveResolveSectionTabModel {
    type View = ActiveResolveSectionTabView;
}
