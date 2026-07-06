mod props;
mod style;
use dioxus::prelude::*;
pub use props::ReasonBadgeProps;
use style::class;
use tw_macro::assert_component;
assert_component!(ReasonBadge);

/// The colour-coded reason badge (Fight / Swap / Spill / Gap pull / Stuck).
#[component]
pub fn ReasonBadge(props: ReasonBadgeProps) -> Element {
    let class = class(props.kind);
    let label = props.label;
    rsx! { span { class, {label} } }
}
