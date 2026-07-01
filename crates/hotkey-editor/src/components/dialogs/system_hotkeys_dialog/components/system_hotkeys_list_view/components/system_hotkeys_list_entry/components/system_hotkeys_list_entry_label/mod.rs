mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::SystemHotkeysListEntryLabelProps;

assert_component!(SystemHotkeysListEntryLabel);

/// The binding name shown in a system-hotkey list row.
#[component]
pub fn SystemHotkeysListEntryLabel(props: SystemHotkeysListEntryLabelProps) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}
