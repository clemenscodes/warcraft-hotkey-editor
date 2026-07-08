mod props;
mod state;
mod style;
use dioxus::prelude::*;
pub use props::ReasonBadgeProps;
pub use state::ReasonBadgeColor;
use style::class;
use tw_macro::assert_component;
assert_component!(ReasonBadge);

/// The base reason badge: a colour-coded pill showing its label. The per-kind
/// wrapper picks the colour; the text arrives as a prop.
#[component]
pub fn ReasonBadge(props: ReasonBadgeProps) -> Element {
    let class = class(props.color);
    let label = props.label;
    rsx! { span { class, {label} } }
}
