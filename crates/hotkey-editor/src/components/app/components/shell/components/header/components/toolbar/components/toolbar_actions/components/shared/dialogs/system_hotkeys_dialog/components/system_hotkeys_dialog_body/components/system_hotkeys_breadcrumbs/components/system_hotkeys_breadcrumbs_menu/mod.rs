pub mod components;
mod model;
pub mod presentation;
mod view;

pub use view::SystemHotkeysBreadcrumbsMenuView;

use components::closed_breadcrumbs_menu::ClosedBreadcrumbsMenu;
use components::open_breadcrumbs_menu::OpenBreadcrumbsMenu;
use dioxus::prelude::*;
use model::SystemHotkeysBreadcrumbsMenuModel;
use presentation::use_system_hotkeys_breadcrumbs_menu;
use tw_macro::assert_component;

/// The category tab list. A pure dispatcher: from the dropdown's open flag it renders
/// the small-viewport popover (`OpenBreadcrumbsMenu`) xor the tab bar
/// (`ClosedBreadcrumbsMenu`). It builds the tab descriptors once — carrying each tab's
/// `menu_open` so the tabs wear the matching look — and hands them to the chosen
/// container.
#[component]
pub fn SystemHotkeysBreadcrumbsMenu(props: SystemHotkeysBreadcrumbsMenuModel) -> Element {
    let tabs = use_system_hotkeys_breadcrumbs_menu(&props);
    if props.is_open {
        rsx! {
            OpenBreadcrumbsMenu {
                tabs,
            }
        }
    } else {
        rsx! {
            ClosedBreadcrumbsMenu {
                tabs,
            }
        }
    }
}

assert_component!(SystemHotkeysBreadcrumbsMenu);
