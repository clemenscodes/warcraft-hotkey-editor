use super::view::FightSectionTabView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FightSectionTabModel {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&FightSectionTabView> for FightSectionTabModel {
    fn from(view: &FightSectionTabView) -> Self {
        let FightSectionTabView {
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

impl ddd::Model for FightSectionTabModel {
    type View = FightSectionTabView;
}
