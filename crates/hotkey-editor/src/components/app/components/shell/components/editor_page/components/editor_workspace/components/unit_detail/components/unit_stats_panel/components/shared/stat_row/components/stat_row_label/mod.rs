mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::StatRowLabelProps;
use style::CLASS;
assert_component!(StatRowLabel);

/// A stat row's label; its regen/primary colour comes from the parent row group.
#[component]
pub fn StatRowLabel(props: StatRowLabelProps) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}
