mod props;
mod style;

use dioxus::prelude::*;
pub use props::MutedStatValueProps;
use style::CLASS;
use tw_macro::assert_component;

/// The muted (zero-figure) value look: faint text, normal weight. Rendered by the
/// [`StatValue`](super::super::StatValue) dispatcher when the figure reports itself
/// muted.
#[component]
pub fn MutedStatValue(props: MutedStatValueProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}

assert_component!(MutedStatValue);
