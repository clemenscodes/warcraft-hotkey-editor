pub mod components;
mod hooks;
mod props;

use components::closed_breadcrumbs_menu::ClosedBreadcrumbsMenu;
use components::open_breadcrumbs_menu::OpenBreadcrumbsMenu;
use dioxus::prelude::*;
use hooks::use_system_hotkeys_breadcrumbs_menu;
use props::SystemHotkeysBreadcrumbsMenuProps;
use tw_macro::assert_component;

/// The category tab list. A pure dispatcher: from the dropdown's open flag it renders
/// the small-viewport popover (`OpenBreadcrumbsMenu`) xor the tab bar
/// (`ClosedBreadcrumbsMenu`). It builds the tab descriptors once — carrying each tab's
/// `menu_open` so the tabs wear the matching look — and hands them to the chosen
/// container.
#[component]
pub fn SystemHotkeysBreadcrumbsMenu(props: SystemHotkeysBreadcrumbsMenuProps) -> Element {
    let tabs = use_system_hotkeys_breadcrumbs_menu(&props);
    if props.is_open {
        rsx! {
            OpenBreadcrumbsMenu { tabs }
        }
    } else {
        rsx! {
            ClosedBreadcrumbsMenu { tabs }
        }
    }
}

assert_component!(SystemHotkeysBreadcrumbsMenu);
