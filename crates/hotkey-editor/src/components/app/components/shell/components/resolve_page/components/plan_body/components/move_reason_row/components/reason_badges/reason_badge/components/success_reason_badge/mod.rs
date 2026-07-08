mod props;
mod style;

use dioxus::prelude::*;
pub use props::SuccessReasonBadgeProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(SuccessReasonBadge);

/// The success-coloured reason badge. Presentational — the dispatcher builds its props and renders it when the
/// colour selects it.
#[component]
pub fn SuccessReasonBadge(props: SuccessReasonBadgeProps) -> Element {
    let label = props.label;
    rsx! {
        span {
            class: CLASS,
            {label}
        }
    }
}
