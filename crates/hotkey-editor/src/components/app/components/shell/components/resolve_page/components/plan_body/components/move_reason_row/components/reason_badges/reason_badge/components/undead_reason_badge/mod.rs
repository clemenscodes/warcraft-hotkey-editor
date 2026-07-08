mod props;
mod style;

use dioxus::prelude::*;
pub use props::UndeadReasonBadgeProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UndeadReasonBadge);

/// The undead-coloured reason badge. Presentational — the dispatcher builds its props and renders it when the
/// colour selects it.
#[component]
pub fn UndeadReasonBadge(props: UndeadReasonBadgeProps) -> Element {
    let label = props.label;
    rsx! {
        span {
            class: CLASS,
            {label}
        }
    }
}
