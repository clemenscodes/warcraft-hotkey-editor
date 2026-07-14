mod model;
mod view;

pub use view::ActiveStatGainView;
mod style;

use dioxus::prelude::*;
use model::ActiveStatGainModel;
use style::CLASS;
use tw_macro::assert_component;

/// The active (non-muted) gain look: green, tabular text. Rendered by the
/// [`StatGain`](super::super::StatGain) dispatcher when the figure is not muted.
#[component]
pub fn ActiveStatGain(props: ActiveStatGainModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(ActiveStatGain);
