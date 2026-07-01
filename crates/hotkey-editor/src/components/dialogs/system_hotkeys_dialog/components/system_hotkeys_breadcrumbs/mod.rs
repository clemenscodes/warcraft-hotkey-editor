pub mod components;
mod hooks;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::system_hotkeys_breadcrumbs_menu::SystemHotkeysBreadcrumbsMenu;
use components::system_hotkeys_breadcrumbs_trigger::SystemHotkeysBreadcrumbsTrigger;
use hooks::use_system_hotkeys_breadcrumbs;
use style::CLASS;

pub use props::SystemHotkeysBreadcrumbsProps;

assert_component!(SystemHotkeysBreadcrumbs);

/// The category bar under the dialog header: a tab row on desktop, a dropdown on
/// small viewports.
#[component]
pub fn SystemHotkeysBreadcrumbs(props: SystemHotkeysBreadcrumbsProps) -> Element {
    let active_category = props.active_category;
    let model = use_system_hotkeys_breadcrumbs(&props);
    rsx! {
        nav {
            class: CLASS,
            aria_label: "System hotkeys categories",
            SystemHotkeysBreadcrumbsTrigger {
                label: model.trigger_label,
                is_open: model.is_open,
                open: model.open_attr,
                on_toggle: model.on_toggle,
            }
            SystemHotkeysBreadcrumbsMenu {
                active_category,
                picker_open: model.open,
                open: model.open_attr,
            }
        }
    }
}
