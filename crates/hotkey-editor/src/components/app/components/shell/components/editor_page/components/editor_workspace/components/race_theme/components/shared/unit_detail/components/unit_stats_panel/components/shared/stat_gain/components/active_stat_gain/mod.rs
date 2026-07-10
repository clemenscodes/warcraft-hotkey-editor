mod props;
mod style;

use dioxus::prelude::*;
pub use props::ActiveStatGainProps;
use style::CLASS;
use tw_macro::assert_component;

/// The active (non-muted) gain look: green, tabular text. Rendered by the
/// [`StatGain`](super::super::StatGain) dispatcher when the figure is not muted.
#[component]
pub fn ActiveStatGain(props: ActiveStatGainProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}

assert_component!(ActiveStatGain);
