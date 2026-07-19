use super::view::InactiveCollisionKindTabView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InactiveCollisionKindTabModel {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&InactiveCollisionKindTabView> for InactiveCollisionKindTabModel {
    fn from(view: &InactiveCollisionKindTabView) -> Self {
        let InactiveCollisionKindTabView {
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

impl ddd::Model for InactiveCollisionKindTabModel {
    type View = InactiveCollisionKindTabView;
}
