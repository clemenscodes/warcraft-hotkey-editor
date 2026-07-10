pub mod components;
mod hooks;
mod props;
mod view;

pub use view::SystemHotkeysBreadcrumbsView;
mod style;

use components::system_hotkeys_breadcrumbs_menu::SystemHotkeysBreadcrumbsMenu;
use components::system_hotkeys_breadcrumbs_trigger::SystemHotkeysBreadcrumbsTrigger;
use dioxus::prelude::*;
use hooks::use_system_hotkeys_breadcrumbs;
use props::SystemHotkeysBreadcrumbsProps;
use style::CLASS;
use tw_macro::assert_component;

/// The category bar under the dialog header: a tab row on desktop, a dropdown on
/// small viewports.
#[component]
pub fn SystemHotkeysBreadcrumbs(props: SystemHotkeysBreadcrumbsProps) -> Element {
    let model = use_system_hotkeys_breadcrumbs(&props);
    let trigger_label = model.trigger_label.clone();
    let trigger_is_open = model.is_open;
    let on_toggle = model.on_toggle;
    let menu_active_category = model.active_category;
    let menu_picker_open = model.open;
    let menu_is_open = model.is_open;
    rsx! {
        nav {
            class: CLASS,
            aria_label: "System hotkeys categories",
            SystemHotkeysBreadcrumbsTrigger {
                label: trigger_label,
                is_open: trigger_is_open,
                on_toggle,
            }
            SystemHotkeysBreadcrumbsMenu {
                active_category: menu_active_category,
                picker_open: menu_picker_open,
                is_open: menu_is_open,
            }
        }
    }
}

assert_component!(SystemHotkeysBreadcrumbs);
