mod props;
mod style;

use super::shared::system_hotkeys_category_tab::SystemHotkeysCategoryTab;
use dioxus::prelude::*;
pub use props::ClosedBreadcrumbsMenuProps;
use style::CLASS;
use tw_macro::assert_component;

/// The closed look of the category menu: the desktop tab bar (hidden on small
/// viewports, where the trigger stands in). Rendered by the dispatcher while the
/// dropdown is closed; owns its listbox root and lays the tabs out in a row.
#[component]
pub fn ClosedBreadcrumbsMenu(props: ClosedBreadcrumbsMenuProps) -> Element {
    let tabs = props.tabs;
    rsx! {
        div { class: CLASS, role: "listbox",
            for tab in tabs {
                SystemHotkeysCategoryTab { ..tab }
            }
        }
    }
}

assert_component!(ClosedBreadcrumbsMenu);
