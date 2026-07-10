pub mod components;
mod hooks;
mod logic;
mod props;
mod style;

use components::system_hotkeys_breadcrumbs_menu::{
    SystemHotkeysBreadcrumbsMenu, SystemHotkeysBreadcrumbsMenuProps,
};
use components::system_hotkeys_breadcrumbs_trigger::{
    SystemHotkeysBreadcrumbsTrigger, SystemHotkeysBreadcrumbsTriggerProps,
};
use dioxus::prelude::*;
use hooks::use_system_hotkeys_breadcrumbs;
pub use props::SystemHotkeysBreadcrumbsProps;
use style::CLASS;
use tw_macro::assert_component;

/// The category bar under the dialog header: a tab row on desktop, a dropdown on
/// small viewports.
#[component]
pub fn SystemHotkeysBreadcrumbs(props: SystemHotkeysBreadcrumbsProps) -> Element {
    let model = use_system_hotkeys_breadcrumbs(&props);
    let trigger = SystemHotkeysBreadcrumbsTriggerProps::from(&model);
    let menu = SystemHotkeysBreadcrumbsMenuProps::from(&model);
    rsx! {
        nav {
            class: CLASS,
            aria_label: "System hotkeys categories",
            SystemHotkeysBreadcrumbsTrigger { ..trigger }
            SystemHotkeysBreadcrumbsMenu { ..menu }
        }
    }
}

assert_component!(SystemHotkeysBreadcrumbs);
