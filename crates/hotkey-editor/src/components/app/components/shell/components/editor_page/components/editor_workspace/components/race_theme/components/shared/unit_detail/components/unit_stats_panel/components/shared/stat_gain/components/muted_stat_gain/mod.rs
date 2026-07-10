mod props;
mod view;

pub use view::MutedStatGainView;
mod style;

use dioxus::prelude::*;
use props::MutedStatGainProps;
use style::CLASS;
use tw_macro::assert_component;

/// The muted (zero-figure) gain look: faint, tabular text. Rendered by the
/// [`StatGain`](super::super::StatGain) dispatcher when the figure reports itself
/// muted.
#[component]
pub fn MutedStatGain(props: MutedStatGainProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}

assert_component!(MutedStatGain);
