mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::SystemHotkeysBreadcrumbsTriggerLabelProps;
use style::CLASS;
assert_component!(SystemHotkeysBreadcrumbsTriggerLabel);

/// The active-category caption shown inside the dropdown trigger.
#[component]
pub fn SystemHotkeysBreadcrumbsTriggerLabel(
    props: SystemHotkeysBreadcrumbsTriggerLabelProps,
) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}
