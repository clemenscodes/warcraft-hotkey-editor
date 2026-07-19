use super::view::InactiveResolveSectionTabView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InactiveResolveSectionTabModel {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&InactiveResolveSectionTabView> for InactiveResolveSectionTabModel {
    fn from(view: &InactiveResolveSectionTabView) -> Self {
        let InactiveResolveSectionTabView {
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

impl ddd::Model for InactiveResolveSectionTabModel {
    type View = InactiveResolveSectionTabView;
}
