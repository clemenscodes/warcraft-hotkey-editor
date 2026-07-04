mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::SystemHotkeysListEntryLabelProps;
use style::CLASS;
assert_component!(SystemHotkeysListEntryLabel);

/// The binding name shown in a system-hotkey list row.
#[component]
pub fn SystemHotkeysListEntryLabel(props: SystemHotkeysListEntryLabelProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}
