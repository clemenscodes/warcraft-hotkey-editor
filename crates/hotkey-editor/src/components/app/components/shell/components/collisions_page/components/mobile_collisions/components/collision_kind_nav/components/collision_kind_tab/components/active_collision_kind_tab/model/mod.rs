use super::view::ActiveCollisionKindTabView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ActiveCollisionKindTabModel {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ActiveCollisionKindTabView> for ActiveCollisionKindTabModel {
    fn from(view: &ActiveCollisionKindTabView) -> Self {
        let ActiveCollisionKindTabView {
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

impl ddd::Model for ActiveCollisionKindTabModel {
    type View = ActiveCollisionKindTabView;
}
