mod props;
mod style;

use super::shared::system_hotkeys_category_tab::SystemHotkeysCategoryTab;
use dioxus::prelude::*;
pub use props::OpenBreadcrumbsMenuProps;
use style::CLASS;
use tw_macro::assert_component;

/// The open look of the category menu: the small-viewport floating popover. Rendered
/// by the dispatcher while the dropdown is open; owns its listbox root and stacks the
/// popover-styled tabs vertically.
#[component]
pub fn OpenBreadcrumbsMenu(props: OpenBreadcrumbsMenuProps) -> Element {
    let tabs = props.tabs;
    rsx! {
        div { class: CLASS, role: "listbox",
            for tab in tabs {
                SystemHotkeysCategoryTab { ..tab }
            }
        }
    }
}

assert_component!(OpenBreadcrumbsMenu);
