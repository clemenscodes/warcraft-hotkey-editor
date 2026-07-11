mod model;
mod view;

pub use view::OpenBreadcrumbsMenuView;
mod style;

use super::shared::system_hotkeys_category_tab::SystemHotkeysCategoryTab;
use dioxus::prelude::*;
use model::OpenBreadcrumbsMenuModel;
use style::CLASS;
use tw_macro::assert_component;

/// The open look of the category menu: the small-viewport floating popover. Rendered
/// by the dispatcher while the dropdown is open; owns its listbox root and stacks the
/// popover-styled tabs vertically.
#[component]
pub fn OpenBreadcrumbsMenu(props: OpenBreadcrumbsMenuModel) -> Element {
    let tabs = props.tabs;
    rsx! {
        div { class: CLASS, role: "listbox",
            for descriptor in tabs {
                SystemHotkeysCategoryTab {
                    category: descriptor.category,
                    is_active: descriptor.is_active,
                    has_separator: descriptor.has_separator,
                    menu_open: descriptor.menu_open,
                    picker_open: descriptor.picker_open,
                }
            }
        }
    }
}

assert_component!(OpenBreadcrumbsMenu);
