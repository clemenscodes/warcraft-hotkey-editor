mod props;
mod style;

use dioxus::prelude::*;
pub use props::SystemHotkeysBreadcrumbsTriggerLabelProps;
use style::CLASS;
use tw_macro::assert_component;
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
