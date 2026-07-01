mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolveReasonBadgeProps;
use style::class;
assert_component!(ResolveReasonBadge);

/// The colour-coded reason badge (Fight / Swap / Spill / Gap pull / Stuck).
#[component]
pub fn ResolveReasonBadge(props: ResolveReasonBadgeProps) -> Element {
    let class = class(props.kind);
    let label = props.label;
    rsx! { span { class, {label} } }
}
