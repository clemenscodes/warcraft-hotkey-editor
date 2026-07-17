use super::view::ActiveSearchScopeOptionView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ActiveSearchScopeOptionModel {
    #[props(into)]
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ActiveSearchScopeOptionView> for ActiveSearchScopeOptionModel {
    fn from(view: &ActiveSearchScopeOptionView) -> Self {
        let ActiveSearchScopeOptionView { label, onclick } = view.clone();
        Self { label, onclick }
    }
}

impl ddd::Model for ActiveSearchScopeOptionModel {
    type View = ActiveSearchScopeOptionView;
}
