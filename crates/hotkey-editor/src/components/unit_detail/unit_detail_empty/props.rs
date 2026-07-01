use dioxus::prelude::*;

/// The message shown in the empty unit-detail card.
#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailEmptyProps {
    #[props(into)]
    pub message: String,
}
