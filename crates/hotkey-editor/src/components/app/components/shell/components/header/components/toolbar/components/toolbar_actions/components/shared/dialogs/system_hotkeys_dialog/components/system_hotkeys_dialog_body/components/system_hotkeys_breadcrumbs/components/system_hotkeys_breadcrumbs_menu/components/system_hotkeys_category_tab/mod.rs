pub mod components;
mod hooks;
mod logic;
mod props;

use components::active_category_tab::{ActiveCategoryTab, ActiveCategoryTabProps};
use components::inactive_category_tab::{InactiveCategoryTab, InactiveCategoryTabProps};
use components::system_hotkeys_category_separator::SystemHotkeysCategorySeparator;
use dioxus::prelude::*;
use hooks::use_system_hotkeys_category_tab;
pub use props::SystemHotkeysCategoryTabProps;
use tw_macro::assert_component;
assert_component!(SystemHotkeysCategoryTab);

/// One selectable category in the breadcrumbs. A pure dispatcher: from the tab's
/// active flag it renders the selected look (`ActiveCategoryTab`) or the dimmed one
/// (`InactiveCategoryTab`), followed by a trailing separator on all but the last.
#[component]
pub fn SystemHotkeysCategoryTab(props: SystemHotkeysCategoryTabProps) -> Element {
    let model = use_system_hotkeys_category_tab(&props);
    let active = ActiveCategoryTabProps::from(&model);
    let inactive = InactiveCategoryTabProps::from(&model);
    let has_separator = model.has_separator;
    let is_active = model.is_active;
    rsx! {
        if is_active {
            ActiveCategoryTab { ..active }
        } else {
            InactiveCategoryTab { ..inactive }
        }
        if has_separator {
            SystemHotkeysCategorySeparator {}
        }
    }
}
