mod model;
mod view;

pub use view::ClosedBreadcrumbsMenuView;
mod style;

use super::shared::system_hotkeys_category_tab::SystemHotkeysCategoryTab;
use dioxus::prelude::*;
use model::ClosedBreadcrumbsMenuModel;
use style::CLASS;
use tw_macro::assert_component;

/// The closed look of the category menu: the desktop tab bar (hidden on small
/// viewports, where the trigger stands in). Rendered by the dispatcher while the
/// dropdown is closed; owns its listbox root and lays the tabs out in a row.
#[component]
pub fn ClosedBreadcrumbsMenu(props: ClosedBreadcrumbsMenuModel) -> Element {
    let tabs = props.tabs;
    rsx! {
        div {
            class: CLASS,
            role: "listbox",
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

assert_component!(ClosedBreadcrumbsMenu);
