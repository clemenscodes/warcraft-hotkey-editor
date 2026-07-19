use super::view::GapPullSectionTabView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GapPullSectionTabModel {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&GapPullSectionTabView> for GapPullSectionTabModel {
    fn from(view: &GapPullSectionTabView) -> Self {
        let GapPullSectionTabView {
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

impl ddd::Model for GapPullSectionTabModel {
    type View = GapPullSectionTabView;
}
