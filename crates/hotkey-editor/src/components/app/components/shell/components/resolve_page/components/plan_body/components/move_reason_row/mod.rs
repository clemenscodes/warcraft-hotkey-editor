pub mod components;
mod props;
mod style;
use components::reason_badge::{ReasonBadge, ReasonBadgeProps};
use dioxus::prelude::*;
pub use props::MoveReasonRowProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(MoveReasonRow);
#[component]
pub fn MoveReasonRow(props: MoveReasonRowProps) -> Element {
    let badge = ReasonBadgeProps::from(&props);
    rsx! {
        div { class: CLASS, ReasonBadge { ..badge } }
    }
}
