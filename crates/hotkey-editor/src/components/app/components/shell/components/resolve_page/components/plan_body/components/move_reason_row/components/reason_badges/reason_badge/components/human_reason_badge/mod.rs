mod props;
mod style;

use dioxus::prelude::*;
pub use props::HumanReasonBadgeProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HumanReasonBadge);

/// The human-coloured reason badge. Presentational — the dispatcher builds its props and renders it when the
/// colour selects it.
#[component]
pub fn HumanReasonBadge(props: HumanReasonBadgeProps) -> Element {
    let label = props.label;
    rsx! {
        span {
            class: CLASS,
            {label}
        }
    }
}
