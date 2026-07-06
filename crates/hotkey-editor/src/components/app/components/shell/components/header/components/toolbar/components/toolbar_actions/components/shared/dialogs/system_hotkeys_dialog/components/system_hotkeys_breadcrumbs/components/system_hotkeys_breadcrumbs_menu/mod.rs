pub mod components;
mod hooks;
mod props;
mod style;

use components::system_hotkeys_category_tab::SystemHotkeysCategoryTab;
use dioxus::prelude::*;
use hooks::use_system_hotkeys_breadcrumbs_menu;
pub use props::SystemHotkeysBreadcrumbsMenuProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(SystemHotkeysBreadcrumbsMenu);

/// The category tab list: the desktop tab bar and the small-viewport popover.
#[component]
pub fn SystemHotkeysBreadcrumbsMenu(props: SystemHotkeysBreadcrumbsMenuProps) -> Element {
    let open = props.open;
    let tabs = use_system_hotkeys_breadcrumbs_menu(&props);
    rsx! {
        div { class: CLASS, role: "listbox", "data-open": open,
            for tab in tabs {
                SystemHotkeysCategoryTab { ..tab }
            }
        }
    }
}
