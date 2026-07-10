use super::view::UnitDetailEmptyView;
use dioxus::prelude::*;

/// The message shown in the empty unit-detail card.
#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailEmptyProps {
    #[props(into)]
    pub message: String,
}

impl From<&UnitDetailEmptyView> for UnitDetailEmptyProps {
    fn from(view: &UnitDetailEmptyView) -> Self {
        let UnitDetailEmptyView { message } = view.clone();
        Self { message }
    }
}

impl ddd::Props for UnitDetailEmptyProps {
    type View = UnitDetailEmptyView;
}
