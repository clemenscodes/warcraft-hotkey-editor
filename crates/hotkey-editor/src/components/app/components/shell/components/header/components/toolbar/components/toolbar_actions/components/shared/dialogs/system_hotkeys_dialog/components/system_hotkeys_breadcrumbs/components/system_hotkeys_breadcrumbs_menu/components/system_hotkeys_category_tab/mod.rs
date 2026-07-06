pub mod components;
mod hooks;
mod props;
mod state;
mod style;

use components::system_hotkeys_category_separator::SystemHotkeysCategorySeparator;
use dioxus::prelude::*;
use hooks::use_system_hotkeys_category_tab;
pub use props::SystemHotkeysCategoryTabProps;
use tw_macro::assert_component;
assert_component!(SystemHotkeysCategoryTab);

/// One selectable category in the breadcrumbs, with a trailing separator on all but
/// the last.
#[component]
pub fn SystemHotkeysCategoryTab(props: SystemHotkeysCategoryTabProps) -> Element {
    let model = use_system_hotkeys_category_tab(&props);
    let class = style::class(model.state);
    let aria_current = if model.is_active { "page" } else { "false" };
    rsx! {
        button {
            class,
            r#type: "button",
            role: "option",
            aria_selected: model
                    .is_active,
            aria_current,
            onclick: model.on_click,
            {model.label}
        }
        if model.has_separator {
            SystemHotkeysCategorySeparator {}
        }
    }
}
