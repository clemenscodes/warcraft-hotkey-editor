mod props;
mod style;

use dioxus::prelude::*;
pub use props::OrcReasonBadgeProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(OrcReasonBadge);

/// The orc-coloured reason badge. Presentational — the dispatcher builds its props and renders it when the
/// colour selects it.
#[component]
pub fn OrcReasonBadge(props: OrcReasonBadgeProps) -> Element {
    let label = props.label;
    rsx! {
        span {
            class: CLASS,
            {label}
        }
    }
}
