use super::view::IdleSearchScopeOptionView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IdleSearchScopeOptionModel {
    #[props(into)]
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&IdleSearchScopeOptionView> for IdleSearchScopeOptionModel {
    fn from(view: &IdleSearchScopeOptionView) -> Self {
        let IdleSearchScopeOptionView { label, onclick } = view.clone();
        Self { label, onclick }
    }
}

impl ddd::Model for IdleSearchScopeOptionModel {
    type View = IdleSearchScopeOptionView;
}
