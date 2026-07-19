use super::view::ResolveSectionTabView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ResolveSectionTabModel {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ResolveSectionTabView> for ResolveSectionTabModel {
    fn from(view: &ResolveSectionTabView) -> Self {
        let ResolveSectionTabView {
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

impl ddd::Model for ResolveSectionTabModel {
    type View = ResolveSectionTabView;
}
