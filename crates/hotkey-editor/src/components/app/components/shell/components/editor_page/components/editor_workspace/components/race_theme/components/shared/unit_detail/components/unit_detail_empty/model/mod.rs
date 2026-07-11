use super::view::UnitDetailEmptyView;
use dioxus::prelude::*;

/// The message shown in the empty unit-detail card.
#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailEmptyModel {
    #[props(into)]
    pub message: String,
}

impl From<&UnitDetailEmptyView> for UnitDetailEmptyModel {
    fn from(view: &UnitDetailEmptyView) -> Self {
        let UnitDetailEmptyView { message } = view.clone();
        Self { message }
    }
}

impl ddd::Model for UnitDetailEmptyModel {
    type View = UnitDetailEmptyView;
}
