pub mod components;
mod hooks;
mod logic;
mod props;

use components::active_category_tab::{ActiveCategoryTab, ActiveCategoryTabProps};
use components::inactive_category_tab::{InactiveCategoryTab, InactiveCategoryTabProps};
use components::popover_active_category_tab::{
    PopoverActiveCategoryTab, PopoverActiveCategoryTabProps,
};
use components::popover_inactive_category_tab::{
    PopoverInactiveCategoryTab, PopoverInactiveCategoryTabProps,
};
use components::system_hotkeys_category_separator::SystemHotkeysCategorySeparator;
use dioxus::prelude::*;
use hooks::use_system_hotkeys_category_tab;
pub use props::SystemHotkeysCategoryTabProps;
use tw_macro::assert_component;

/// One selectable category in the breadcrumbs. A pure dispatcher: from the tab's
/// popover and active flags it renders the matching look — the tab-bar
/// `ActiveCategoryTab` xor `InactiveCategoryTab`, or the popover
/// `PopoverActiveCategoryTab` xor `PopoverInactiveCategoryTab` — followed by a
/// trailing separator on all but the last (only in the tab bar; the popover has none).
#[component]
pub fn SystemHotkeysCategoryTab(props: SystemHotkeysCategoryTabProps) -> Element {
    let model = use_system_hotkeys_category_tab(&props);
    let active = ActiveCategoryTabProps::from(&model);
    let inactive = InactiveCategoryTabProps::from(&model);
    let popover_active = PopoverActiveCategoryTabProps::from(&model);
    let popover_inactive = PopoverInactiveCategoryTabProps::from(&model);
    let menu_open = model.menu_open;
    let is_active = model.is_active;
    let has_separator = model.has_separator && !menu_open;
    rsx! {
        if menu_open {
            if is_active {
                PopoverActiveCategoryTab { ..popover_active }
            } else {
                PopoverInactiveCategoryTab { ..popover_inactive }
            }
        } else if is_active {
            ActiveCategoryTab { ..active }
        } else {
            InactiveCategoryTab { ..inactive }
        }
        if has_separator {
            SystemHotkeysCategorySeparator {}
        }
    }
}

assert_component!(SystemHotkeysCategoryTab);
