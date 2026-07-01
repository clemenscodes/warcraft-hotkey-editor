pub mod components;
mod props;
mod style;
use components::resolve_reason_badge::{ResolveReasonBadge, ResolveReasonBadgeProps};
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolveMoveReasonRowProps;
use style::CLASS;
assert_component!(ResolveMoveReasonRow);
#[component]
pub fn ResolveMoveReasonRow(props: ResolveMoveReasonRowProps) -> Element {
    let badge = ResolveReasonBadgeProps::from(&props);
    rsx! {
        div { class: CLASS, ResolveReasonBadge { ..badge } }
    }
}
