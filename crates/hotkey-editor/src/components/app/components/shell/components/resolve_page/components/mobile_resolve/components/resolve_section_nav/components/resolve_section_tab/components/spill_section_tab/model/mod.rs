use super::view::SpillSectionTabView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SpillSectionTabModel {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&SpillSectionTabView> for SpillSectionTabModel {
    fn from(view: &SpillSectionTabView) -> Self {
        let SpillSectionTabView {
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

impl ddd::Model for SpillSectionTabModel {
    type View = SpillSectionTabView;
}
