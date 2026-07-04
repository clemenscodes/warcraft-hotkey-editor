pub mod components;
mod props;
mod style;
use crate::assert_component;
use components::reason_badge::{ReasonBadge, ReasonBadgeProps};
use dioxus::prelude::*;
pub use props::MoveReasonRowProps;
use style::CLASS;
assert_component!(MoveReasonRow);
#[component]
pub fn MoveReasonRow(props: MoveReasonRowProps) -> Element {
    let badge = ReasonBadgeProps::from(&props);
    rsx! {
        div { class: CLASS, ReasonBadge { ..badge } }
    }
}
