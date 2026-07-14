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
