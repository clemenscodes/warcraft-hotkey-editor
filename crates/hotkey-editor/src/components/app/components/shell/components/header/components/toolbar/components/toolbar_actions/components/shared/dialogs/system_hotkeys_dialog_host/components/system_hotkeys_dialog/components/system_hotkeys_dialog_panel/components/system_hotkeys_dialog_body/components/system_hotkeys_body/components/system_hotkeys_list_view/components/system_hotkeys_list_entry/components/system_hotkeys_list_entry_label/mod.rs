mod props;
mod style;

use dioxus::prelude::*;
pub use props::SystemHotkeysListEntryLabelProps;
use style::CLASS;
use tw_macro::assert_component;

/// The binding name shown in a system-hotkey list row.
#[component]
pub fn SystemHotkeysListEntryLabel(props: SystemHotkeysListEntryLabelProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}

assert_component!(SystemHotkeysListEntryLabel);
