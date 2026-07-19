use super::view::SwapSectionTabView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SwapSectionTabModel {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&SwapSectionTabView> for SwapSectionTabModel {
    fn from(view: &SwapSectionTabView) -> Self {
        let SwapSectionTabView {
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

impl ddd::Model for SwapSectionTabModel {
    type View = SwapSectionTabView;
}
