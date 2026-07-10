mod props;
mod style;

use dioxus::prelude::*;
use props::SystemHotkeysBreadcrumbsTriggerLabelProps;
use style::CLASS;
use tw_macro::assert_component;

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

assert_component!(SystemHotkeysBreadcrumbsTriggerLabel);
