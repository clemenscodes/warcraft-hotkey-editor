pub mod components;
mod hooks;
mod props;

use components::active_category_tab::ActiveCategoryTab;
use components::inactive_category_tab::InactiveCategoryTab;
use components::popover_active_category_tab::PopoverActiveCategoryTab;
use components::popover_inactive_category_tab::PopoverInactiveCategoryTab;
use components::system_hotkeys_category_separator::SystemHotkeysCategorySeparator;
use dioxus::prelude::*;
use hooks::use_system_hotkeys_category_tab;
use props::SystemHotkeysCategoryTabProps;
use tw_macro::assert_component;

/// One selectable category in the breadcrumbs. A pure dispatcher: from the tab's
/// popover and active flags it renders the matching look — the tab-bar
/// `ActiveCategoryTab` xor `InactiveCategoryTab`, or the popover
/// `PopoverActiveCategoryTab` xor `PopoverInactiveCategoryTab` — followed by a
/// trailing separator on all but the last (only in the tab bar; the popover has none).
#[component]
pub fn SystemHotkeysCategoryTab(props: SystemHotkeysCategoryTabProps) -> Element {
    let model = use_system_hotkeys_category_tab(&props);
    let menu_open = model.menu_open;
    let is_active = model.is_active;
    let has_separator = model.has_separator && !menu_open;
    let on_click = model.on_click;
    rsx! {
        if menu_open {
            if is_active {
                PopoverActiveCategoryTab {
                    label: model.label.clone(),
                    on_click,
                }
            } else {
                PopoverInactiveCategoryTab {
                    label: model.label.clone(),
                    on_click,
                }
            }
        } else if is_active {
            ActiveCategoryTab {
                label: model.label.clone(),
                on_click,
            }
        } else {
            InactiveCategoryTab {
                label: model.label.clone(),
                on_click,
            }
        }
        if has_separator {
            SystemHotkeysCategorySeparator {}
        }
    }
}

assert_component!(SystemHotkeysCategoryTab);
