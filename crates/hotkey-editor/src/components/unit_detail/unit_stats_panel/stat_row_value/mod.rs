mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::StatRowValueProps;
use style::CLASS;
assert_component!(StatRowValue);

/// A stat row's value; hp/mana colour comes from the parent row group, and the
/// zero state mutes it.
#[component]
pub fn StatRowValue(props: StatRowValueProps) -> Element {
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
