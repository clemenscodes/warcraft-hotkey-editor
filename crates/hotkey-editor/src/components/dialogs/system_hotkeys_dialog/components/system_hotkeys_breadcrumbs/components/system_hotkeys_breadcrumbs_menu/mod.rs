pub mod components;
mod hooks;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::system_hotkeys_category_tab::SystemHotkeysCategoryTab;
use hooks::use_system_hotkeys_breadcrumbs_menu;
use style::CLASS;

pub use props::SystemHotkeysBreadcrumbsMenuProps;

assert_component!(SystemHotkeysBreadcrumbsMenu);

/// The category tab list: the desktop tab bar and the small-viewport popover.
#[component]
pub fn SystemHotkeysBreadcrumbsMenu(props: SystemHotkeysBreadcrumbsMenuProps) -> Element {
    let open = props.open;
    let tabs = use_system_hotkeys_breadcrumbs_menu(&props);
    rsx! {
        div {
            class: CLASS,
            role: "listbox",
            "data-open": open,
            for tab in tabs {
                SystemHotkeysCategoryTab { ..tab }
            }
        }
    }
}
