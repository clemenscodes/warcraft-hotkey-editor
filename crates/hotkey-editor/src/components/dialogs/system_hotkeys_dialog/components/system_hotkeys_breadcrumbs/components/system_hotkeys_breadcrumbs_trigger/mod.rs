pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::system_hotkeys_breadcrumbs_trigger_caret::SystemHotkeysBreadcrumbsTriggerCaret;
use components::system_hotkeys_breadcrumbs_trigger_label::SystemHotkeysBreadcrumbsTriggerLabel;
use style::CLASS;

pub use props::SystemHotkeysBreadcrumbsTriggerProps;

assert_component!(SystemHotkeysBreadcrumbsTrigger);

/// The small-viewport dropdown trigger showing the active category.
#[component]
pub fn SystemHotkeysBreadcrumbsTrigger(props: SystemHotkeysBreadcrumbsTriggerProps) -> Element {
    let label = props.label;
    let is_open = props.is_open;
    let open = props.open;
    let on_toggle = props.on_toggle;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_haspopup: "listbox",
            aria_expanded: is_open,
            onclick: on_toggle,
            SystemHotkeysBreadcrumbsTriggerLabel { text: label }
            SystemHotkeysBreadcrumbsTriggerCaret { open }
        }
    }
}
