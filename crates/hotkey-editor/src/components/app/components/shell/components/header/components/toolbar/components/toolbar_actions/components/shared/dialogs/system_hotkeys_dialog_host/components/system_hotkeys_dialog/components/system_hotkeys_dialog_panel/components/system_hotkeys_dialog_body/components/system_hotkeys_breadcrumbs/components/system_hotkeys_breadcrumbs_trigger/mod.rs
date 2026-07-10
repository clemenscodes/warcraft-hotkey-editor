pub mod components;
mod props;
mod style;

use components::system_hotkeys_breadcrumbs_trigger_caret::{
    SystemHotkeysBreadcrumbsTriggerCaret, SystemHotkeysBreadcrumbsTriggerCaretProps,
};
use components::system_hotkeys_breadcrumbs_trigger_label::{
    SystemHotkeysBreadcrumbsTriggerLabel, SystemHotkeysBreadcrumbsTriggerLabelProps,
};
use dioxus::prelude::*;
pub use props::SystemHotkeysBreadcrumbsTriggerProps;
use style::CLASS;
use tw_macro::assert_component;

/// The small-viewport dropdown trigger showing the active category.
#[component]
pub fn SystemHotkeysBreadcrumbsTrigger(props: SystemHotkeysBreadcrumbsTriggerProps) -> Element {
    let caption = SystemHotkeysBreadcrumbsTriggerLabelProps::from(&props);
    let caret = SystemHotkeysBreadcrumbsTriggerCaretProps::from(&props);
    let is_open = props.is_open;
    let on_toggle = props.on_toggle;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_haspopup: "listbox",
            aria_expanded: is_open,
            onclick: on_toggle,
            SystemHotkeysBreadcrumbsTriggerLabel { ..caption }
            SystemHotkeysBreadcrumbsTriggerCaret { ..caret }
        }
    }
}

assert_component!(SystemHotkeysBreadcrumbsTrigger);
