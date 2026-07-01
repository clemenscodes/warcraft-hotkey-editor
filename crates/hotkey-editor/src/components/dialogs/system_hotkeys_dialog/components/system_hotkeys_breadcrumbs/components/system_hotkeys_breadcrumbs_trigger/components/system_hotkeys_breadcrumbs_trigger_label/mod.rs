mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::SystemHotkeysBreadcrumbsTriggerLabelProps;

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
