pub mod components;
mod props;
mod style;

use components::system_hotkeys_breadcrumbs_trigger_caret::SystemHotkeysBreadcrumbsTriggerCaret;
use components::system_hotkeys_breadcrumbs_trigger_label::SystemHotkeysBreadcrumbsTriggerLabel;
use dioxus::prelude::*;
use props::SystemHotkeysBreadcrumbsTriggerProps;
use style::CLASS;
use tw_macro::assert_component;

/// The small-viewport dropdown trigger showing the active category.
#[component]
pub fn SystemHotkeysBreadcrumbsTrigger(props: SystemHotkeysBreadcrumbsTriggerProps) -> Element {
    let label_text = props.label.clone();
    let caret_is_open = props.is_open;
    let is_open = props.is_open;
    let on_toggle = props.on_toggle;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_haspopup: "listbox",
            aria_expanded: is_open,
            onclick: on_toggle,
            SystemHotkeysBreadcrumbsTriggerLabel { text: label_text }
            SystemHotkeysBreadcrumbsTriggerCaret { is_open: caret_is_open }
        }
    }
}

assert_component!(SystemHotkeysBreadcrumbsTrigger);
