mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::StatRowGainProps;
use style::CLASS;
assert_component!(StatRowGain);

/// A stat row's gain figure; regen alignment/colour comes from the parent row
/// group, and the zero state mutes it.
#[component]
pub fn StatRowGain(props: StatRowGainProps) -> Element {
    let text = props.text;
    let is_zero = props.is_zero;
    rsx! {
        span {
            class: CLASS,
            "data-zero": is_zero,
            {text}
        }
    }
}
